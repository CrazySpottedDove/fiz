use crate::{
    material::{MATERIALS, RECORD},
    session::{Session, SESSION},
    utils::{Dir, Store, CONFIG, CONFIG_DIR},
};
use anyhow::Result;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tauri::{AppHandle, Emitter};
#[derive(Serialize, Deserialize, Clone)]
pub struct Watch {
    pub id: u64,
    pub name: String,
}
use crate::utils::Load;
impl Dir for Vec<Watch> {
    fn dir() -> std::path::PathBuf {
        CONFIG_DIR.join("watches.json")
    }
}
impl Load for Vec<Watch> {}
impl Store for Vec<Watch> {}
lazy_static! {
    pub static ref WATCHES: Mutex<Vec<Watch>> = Mutex::new(Vec::<Watch>::load());
}

impl Session {
    pub async fn get_watches(&self) -> Result<()> {
        let watches = WATCHES.lock().await.clone();
        let materials = MATERIALS.lock().await.clone();
        let record = RECORD.lock().await.clone();
        let accept_mp4 = CONFIG.read().unwrap().accept_mp4;
        for watch in watches {
            let materials_watched = materials.get(&watch.id).unwrap();
            for material in materials_watched {
                for upload in &material.uploads {
                    if record.contains(&upload.id) {
                        continue;
                    }
                    if !accept_mp4 && upload.name.contains("mp4") {
                        continue;
                    }
                    self.fetch_upload(upload.id, &watch.name, &upload.name)
                        .await?;
                }
            }
        }
        Ok(())
    }
}

#[tauri::command]
pub async fn get_watches() -> Result<(), String> {
    SESSION.get_watches().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn init_watches(app: AppHandle) -> Result<(), String> {
    let watches = &*WATCHES.lock().await;
    app.emit("watches-inited", watches)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn set_watches(watches: Vec<Watch>) -> Result<(), String> {
    *WATCHES.lock().await = watches;
    SESSION.get_watches().await.map_err(|e| e.to_string())?;
    Ok(())
}
