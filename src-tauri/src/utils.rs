use crate::account::ACCOUNT;
use crate::courseware::COURSES;
use crate::grade::{ANALYSIS, GRADES};
use crate::homework::HOMEWORKS;
use crate::material::{MATERIALS, RECORD};
use crate::session::SESSION;
use crate::watch::WATCHES;
use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
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
#[serde(default)]
pub struct Config {
    pub courseware_dir: PathBuf,
    pub exp: bool,
    pub accept_mp4: bool,
    pub material_rev: bool,
    pub show_finished_task: bool,
    pub show_grade: bool,
    pub pdf:bool,
}

impl Store for Config {}

pub trait Store: Serialize + Dir {
    fn store(&self) -> Result<(), String> {
        let path = Self::dir();
        let data = serde_json::to_string(self).map_err(|e| e.to_string())?;
        fs::write(path, data).map_err(|e| e.to_string())?;
        Ok(())
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            courseware_dir: FIZ_DIR.join("courseware"),
            exp: false,
            accept_mp4: true,
            material_rev: true,
            show_finished_task: false,
            show_grade: true,
            pdf:false,
        }
    }
}
impl Load for Config {}
impl Dir for Config {
    fn dir() -> PathBuf {
        CONFIG_DIR.join("config.json")
    }
}
pub trait Dir {
    fn dir() -> PathBuf;
}

pub trait Load: DeserializeOwned + Default + Dir {
    fn load() -> Self {
        let path = Self::dir();
        if !path.exists() {
            return Self::default();
        }
        let Ok(file) = fs::File::open(&path) else {
            return Self::default();
        };
        let Ok(data) = serde_json::from_reader(file) else {
            return Self::default();
        };
        data
    }
}
#[tauri::command]
pub fn check_dir() -> Result<(), String> {
    if !CONFIG_DIR.exists() {
        std::fs::create_dir_all(CONFIG_DIR.as_path()).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn store() -> Result<(), String> {
    ACCOUNT.lock().await.store()?;
    SESSION.store()?;
    COURSES.lock().await.store()?;
    CONFIG.read().unwrap().store()?;
    GRADES.lock().await.store()?;
    ANALYSIS.lock().await.store()?;
    MATERIALS.lock().await.store()?;
    RECORD.lock().await.store()?;
    HOMEWORKS.lock().await.store()?;
    WATCHES.lock().await.store()?;
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
pub fn update_config(new_config:Config) {
    let mut config = CONFIG.write().unwrap();
    *config = new_config;
}
