use crate::session::{Session, HOME_URL, LOGIN_URL, MAX_RETRIES, PUBKEY_URL, SESSION};
use crate::utils::{rsa_no_padding, Dir, Load, Store, CONFIG_DIR};
use anyhow::{anyhow, Result};
use futures::join;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
const ZDBK_URL :&str="https://zjuam.zju.edu.cn/cas/login?service=https://zdbk.zju.edu.cn/jwglxt/xtgl/login_ssologin.html";
pub const ETA_URL: &str = "https://eta.zju.edu.cn/index/student";
lazy_static! {
    // 用Mutex包装，这样可以获取可变引用进行修改
    pub static ref ACCOUNT: Mutex<Account> = Mutex::new(Account::load());
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub stuid: String,
    pub password: String,
    pub valid: bool,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            stuid: "".to_string(),
            password: "".to_string(),
            valid: false,
        }
    }
}

impl Dir for Account {
    fn dir() -> std::path::PathBuf {
        CONFIG_DIR.join("account.json")
    }
}

impl Load for Account {}
impl Store for Account {}

#[tauri::command]
pub async fn check_account() -> bool {
    ACCOUNT.lock().await.valid
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

            let (res_home, res_zdbk, res_eta) = join!(
                self.client.get(HOME_URL).send(),
                self.client.get(ZDBK_URL).send(),
                self.client.get(ETA_URL).send(),
            );
            res_home?;
            res_zdbk?;
            if let Err(_) = res_eta {
                println!("ETA_URL: {}", ETA_URL);
            };
            account.valid = true;
            return Ok(());
        }
        Ok(())
    }
    pub async fn login(&self, account: &mut Account) -> Result<()> {
        let res_home = self.client.get(HOME_URL).send().await?;
        if res_home.url().query() == None {
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

    pub async fn login_zdbk(&self) -> Result<()> {
        self.client.get(ZDBK_URL).send().await?;
        Ok(())
    }

    pub async fn login_eta(&self) -> Result<()> {
        self.client.get(ETA_URL).send().await?;
        Ok(())
    }
}

#[tauri::command]
pub async fn login(app: AppHandle) -> Result<(), String> {
    let mut account = ACCOUNT.lock().await;
    SESSION
        .login(&mut account)
        .await
        .map_err(|e| e.to_string())?;

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

    *ACCOUNT.lock().await = account;
    app.emit("login-success", ()).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn login_zdbk() -> Result<(), String> {
    SESSION.login_zdbk().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn login_eta() -> Result<(), String> {
    SESSION.login_eta().await.map_err(|e| e.to_string())?;
    Ok(())
}
