use crate::utils::{Dir, CONFIG_DIR};
use anyhow::Result;
use cookie_store::CookieStore;

use lazy_static::lazy_static;

use reqwest::header::{HeaderMap, USER_AGENT};
use reqwest::Client;
use reqwest_cookie_store::CookieStoreMutex;

use std::fs;
use std::sync::Arc;
pub const MAX_RETRIES: usize = 3;
pub const HOME_URL: &str = "https://courses.zju.edu.cn";
pub const PUBKEY_URL: &str = "https://zjuam.zju.edu.cn/cas/v2/getPubKey";
pub const LOGIN_URL: &str = "https://zjuam.zju.edu.cn/cas/login";

lazy_static! {
    pub static ref SESSION: Session = Session::load();
}

pub struct Session {
    pub cookie_store: Arc<CookieStoreMutex>,
    pub client: Client,
}

impl Dir for Session {
    fn dir() -> std::path::PathBuf {
        CONFIG_DIR.join("cookie.json")
    }
}

impl Session {
    pub fn load() -> Self {
        let cookie_dir = Session::dir();
        #[allow(deprecated)]
        let cookie_store = if cookie_dir.exists() {
            CookieStore::load_json(std::io::BufReader::new(
                std::fs::File::open(cookie_dir).unwrap(),
            ))
            .unwrap_or_default()
        } else {
            CookieStore::default()
        };
        let cookie_store = Arc::new(CookieStoreMutex::new(cookie_store));
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            "Mozilla/5.0 (X11; Linux x86_64; rv:88.0) Gecko/20100101 Firefox/88.0"
                .parse()
                .unwrap(),
        );

        let client = Client::builder()
            .cookie_provider(cookie_store.clone())
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(1200))
            .no_proxy()
            .build()
            .unwrap();
        Self {
            cookie_store,
            client,
        }
    }
}

impl Session {
    pub fn store(&self) -> Result<(), String> {
        let cookie_dir = Self::dir();
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(cookie_dir)
            .map_err(|e| e.to_string())?;
        let store = self.cookie_store.lock().unwrap();
        #[allow(deprecated)]
        store.save_json(&mut file).map_err(|e| e.to_string())?;
        Ok(())
    }
}
