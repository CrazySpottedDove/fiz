use crate::session::{Session, GRADER_URL, HOME_URL, LOGIN_URL, MAX_RETRIES, PUBKEY_URL, SESSION};
use crate::utils::{rsa_no_padding, Store, CONFIG_DIR};
use anyhow::{anyhow, Result};
use futures::join;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};
lazy_static! {
    // 用Mutex包装，这样可以获取可变引用进行修改
    pub static ref ACCOUNT: Mutex<Option<Account>> = Mutex::new(load_account());
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub stuid: String,
    pub password: String,
    pub valid: bool,
}

fn load_account() -> Option<Account> {
    let account_dir = CONFIG_DIR.join("account.json");
    if !account_dir.exists() {
        return None;
    }
    let Ok(account) = fs::read_to_string(account_dir) else {
        return None;
    };
    let Ok(account) = serde_json::from_str::<Account>(&account) else {
        return None;
    };
    if !account.valid {
        return None;
    }
    Some(account)
}

#[tauri::command]
pub fn check_account() -> bool {
    ACCOUNT.lock().unwrap().is_some()
}

impl Store for Account {
    fn store(&self) -> Result<(), String> {
        let account_dir = CONFIG_DIR.join("account.json");
        let account_str = serde_json::to_string(self).map_err(|e| e.to_string())?;
        fs::write(account_dir, account_str).map_err(|e| e.to_string())?;
        Ok(())
    }
}
impl Account {
    pub fn new(stuid: String, password: String) -> Self {
        Self {
            stuid,
            password,
            valid: false,
        }
    }
}
impl Session {
    async fn login_core(&self, account: &mut Account) -> Result<()> {
        async fn get_execution(session: &Session) -> Result<String> {
            let res_login_page = session.client.get(LOGIN_URL).send().await?;
            let text = res_login_page.text().await?;
            let re =
                regex::Regex::new(r#"<input type="hidden" name="execution" value="(.*?)" />"#)?;
            let execution = re
                .captures(&text)
                .and_then(|cap| cap.get(1).map(|m| m.as_str()))
                .ok_or(anyhow!("Execution value not found"))?;
            Ok(execution.to_string())
        }
        async fn get_pubkey(session: &Session) -> Result<(String, String)> {
            let res_pubkey = session.client.get(PUBKEY_URL).send().await?;
            let json: Value = res_pubkey.json().await?;
            let modulus = json["modulus"]
                .as_str()
                .ok_or(anyhow!("MODULUS NOT FOUND"))?;
            let exponent = json["exponent"]
                .as_str()
                .ok_or(anyhow!("EXPONENT NOT FOUND"))?;
            Ok((modulus.to_string(), exponent.to_string()))
        }
        for retry in 1..MAX_RETRIES {
            let (res_execution, res_pubkey) = join!(get_execution(self), get_pubkey(self));
            let execution = res_execution?;
            let (modulus, exponent) = res_pubkey?;
            let rsapwd = rsa_no_padding(&account.password, &modulus, &exponent);
            let params = [
                ("username", account.stuid.as_str()),
                ("password", &rsapwd),
                ("execution", &execution),
                ("_eventId", "submit"),
                ("authcode", ""),
                ("rememberMe", "true"),
            ];
            let res_login = self.client.post(LOGIN_URL).form(&params).send().await?;

            if res_login.url().to_string().contains(LOGIN_URL) {
                if retry == MAX_RETRIES - 1 {
                    account.valid = false;
                    return Err(anyhow!("请检查学号-密码正确性及你的网络连接状态"));
                }
                continue;
            }

            let (res_home, res_grader) = join!(
                self.client.get(HOME_URL).send(),
                self.client.get(GRADER_URL).send()
            );
            res_home?;
            res_grader?;
            account.valid = true;
            return Ok(());
        }
        Ok(())
    }
    pub async fn login(&self, account: &mut Account) -> Result<()> {
        let (res_home, res_grader) = join!(
            self.client.get(HOME_URL).send(),
            self.client.get(GRADER_URL).send()
        );
        let res_home = res_home?;
        let res_grader = res_grader?;
        if res_home.url().query() == None && res_grader.url().query() == None {
            account.valid = true;
            return Ok(());
        }
        self.login_core(account).await?;
        Ok(())
    }

    pub async fn relogin(&self, account: &mut Account) -> Result<()> {
        self.cookie_store.lock().unwrap().clear();
        self.login_core(account).await?;
        Ok(())
    }
}

#[tauri::command]
pub async fn login(app: AppHandle) -> Result<(), String> {
    let mut account;
    {
        let mut account_guard = ACCOUNT.lock().map_err(|e| e.to_string())?;
        account = account_guard.take().ok_or("ACCOUNT NOT INITIALIZED")?;
    }
    SESSION
        .login(&mut account)
        .await
        .map_err(|e| e.to_string())?;

    let mut account_guard = ACCOUNT.lock().map_err(|e| e.to_string())?;
    *account_guard = Some(account);
    app.emit("login-success", ()).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn relogin(stuid: String, password: String, app: AppHandle) -> Result<(), String> {
    let mut account = Account::new(stuid, password);
    SESSION
        .relogin(&mut account)
        .await
        .map_err(|e| e.to_string())?;

    let mut account_guard = ACCOUNT.lock().map_err(|e| e.to_string())?;
    *account_guard = Some(account);
    app.emit("login-success", ()).map_err(|e| e.to_string())?;
    println!("Relogin success");
    Ok(())
}
