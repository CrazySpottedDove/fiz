use crate::account::ACCOUNT;
use crate::courseware::COURSES;
use crate::grade::{ANALYSIS, GRADES};
use crate::homework::HOMEWORKS;
use crate::material::{MATERIALS, RECORD};
use crate::session::SESSION;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::RwLock;
use tauri::{AppHandle, Emitter};
lazy_static! {
    static ref FIZ_DIR: PathBuf = get_fiz_dir();
    pub static ref CONFIG_DIR: PathBuf = FIZ_DIR.join(".config");
    pub static ref CONFIG: RwLock<Config> = RwLock::new(Config::load());
}
pub fn get_fiz_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    let home_dir = std::env::var("USERPROFILE").unwrap();
    #[cfg(target_os = "linux")]
    let home_dir = std::env::var("HOME").unwrap();
    #[cfg(target_os = "macos")]
    let home_dir = std::env::var("HOME").unwrap();
    let fiz_dir = PathBuf::from(home_dir).join(".fiz");
    if !fiz_dir.exists() {
        fs::create_dir(&fiz_dir).unwrap();
    }
    fiz_dir
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub courseware_dir: PathBuf,
    pub exp: bool,
    pub accept_mp4: bool,
}

impl Store for Config {
    fn store(&self) -> Result<(), String> {
        let config_dir = CONFIG_DIR.join("config.json");
        let config_str = serde_json::to_string(self).map_err(|e| e.to_string())?;
        fs::write(config_dir, config_str).map_err(|e| e.to_string())?;
        Ok(())
    }
}
impl Config {
    pub fn new() -> Self {
        Self {
            courseware_dir: FIZ_DIR.join("courseware"),
            exp: false,
            accept_mp4: true,
        }
    }
}
pub trait Store {
    fn store(&self) -> Result<(), String>;
}
impl Load for Config {
    fn load() -> Self {
        let config_dir = CONFIG_DIR.join("config.json");
        if config_dir.exists() {
            let Ok(reader) = std::fs::File::open(config_dir) else {
                return Self::new();
            };
            let Ok(config) = serde_json::from_reader(reader) else {
                return Self::new();
            };
            config
        } else {
            Self::new()
        }
    }
}
pub trait Load {
    fn load() -> Self;
}

#[tauri::command]
pub fn check_dir() -> Result<(), String> {
    if !CONFIG_DIR.exists() {
        std::fs::create_dir_all(CONFIG_DIR.as_path()).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn store() -> Result<(), String> {
    if let Some(account) = ACCOUNT.lock().unwrap().as_ref() {
        account.store()?;
    }
    SESSION.store()?;
    COURSES.lock().unwrap().store()?;
    CONFIG.read().unwrap().store()?;
    GRADES.lock().unwrap().store()?;
    ANALYSIS.lock().unwrap().store()?;
    MATERIALS.lock().unwrap().store()?;
    RECORD.lock().unwrap().store()?;
    HOMEWORKS.lock().unwrap().store()?;
    Ok(())
}

pub fn rsa_no_padding(src: &str, modulus: &str, exponent: &str) -> String {
    let m = num::BigUint::parse_bytes(modulus.as_bytes(), 16).unwrap();
    let e = num::BigUint::parse_bytes(exponent.as_bytes(), 16).unwrap();

    let input_nr = num::BigUint::from_bytes_be(src.as_bytes());

    let crypt_nr = input_nr.modpow(&e, &m);

    crypt_nr
        .to_bytes_be()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect()
}

#[tauri::command]
pub fn init_config(app: AppHandle) -> Result<(), String> {
    app.emit("config-inited", &*CONFIG)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_config(courseware_dir: PathBuf, exp: bool, accept_mp4: bool) {
    let mut config = CONFIG.write().unwrap();
    config.courseware_dir = courseware_dir;
    config.exp = exp;
    config.accept_mp4 = accept_mp4;
}
