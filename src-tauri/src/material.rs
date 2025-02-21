use crate::{
    courseware::COURSES,
    session::{Session, MAX_RETRIES, SESSION},
    utils::{Load, Store, CONFIG, CONFIG_DIR},
};
use anyhow::{anyhow, Result};
use futures::{future::join_all, StreamExt};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};
use tauri_plugin_opener::OpenerExt;
use tokio::{fs::File, io::AsyncWriteExt};
#[derive(Serialize, Deserialize, Clone)]
pub struct Upload {
    pub id: u64,
    pub reference_id: u64,
    pub name: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Material {
    pub id: u64,
    pub title: String,
    pub uploads: Vec<Upload>,
}

impl Load for HashMap<u64, Vec<Material>> {
    fn load() -> Self {
        let materials_dir = CONFIG_DIR.join("materials.json");
        if materials_dir.exists() {
            let Ok(reader) = std::fs::File::open(materials_dir) else {
                return HashMap::new();
            };
            let Ok(materials) = serde_json::from_reader(reader) else {
                return HashMap::new();
            };
            materials
        } else {
            HashMap::new()
        }
    }
}

impl Store for HashMap<u64, Vec<Material>> {
    fn store(&self) -> Result<(), String> {
        let materials_dir = CONFIG_DIR.join("materials.json");
        let materials_str = serde_json::to_string(self).map_err(|e| e.to_string())?;
        std::fs::write(materials_dir, materials_str).map_err(|e| e.to_string())?;
        Ok(())
    }
}

lazy_static! {
    pub static ref MATERIALS: Mutex<HashMap<u64, Vec<Material>>> =
        Mutex::new(HashMap::<u64, Vec<Material>>::load());
}

impl Upload {
    pub fn new(id: u64, reference_id: u64, name: String) -> Self {
        Self {
            id,
            reference_id,
            name,
        }
    }
}
impl Session {
    pub async fn get_materials_by_course_id(&self, id: u64) -> Result<Vec<Material>> {
        let url = format!("https://courses.zju.edu.cn/api/course/{id}/coursewares?conditions=%7B%22category%22:null,%22class_ids%22:%5B%5D,%22itemsSortBy%22:%7B%22predicate%22:%22chapter%22,%22reverse%22:false%7D,%22ignore_activity_types%22:%5B%22lesson%22%5D%7D&page=1&page_size=1000");
        for _ in 1..MAX_RETRIES {
            let Ok(res) = self.client.get(&url).send().await else {
                continue;
            };
            let Ok(json) = res.json::<Value>().await else {
                continue;
            };
            let Some(materials) = json["activities"].as_array() else {
                continue;
            };
            let mut material_vec = Vec::new();
            for material in materials {
                let Some(upload_array) = material["uploads"].as_array() else {
                    continue;
                };
                let id = material["id"].as_u64().unwrap();
                let title = material["title"].as_str().unwrap().to_string();
                let uploads = upload_array
                    .iter()
                    .map(|upload| {
                        let id = upload["id"].as_u64().unwrap();
                        let reference_id = upload["reference_id"].as_u64().unwrap();
                        let name = upload["name"]
                            .as_str()
                            .unwrap()
                            .to_string()
                            .trim()
                            .to_string();
                        Upload::new(id, reference_id, name)
                    })
                    .collect();
                let material = Material { id, title, uploads };
                material_vec.push(material);
            }
            return Ok(material_vec);
        }

        Err(anyhow!("获取课件失败"))
    }
    pub async fn get_materials(&self) -> Result<()> {
        // 克隆一下 courses，避免持有锁太久
        let courses = COURSES.lock().unwrap().clone();

        // 为每个课程创建一个异步任务，获取活动信息
        let futures = courses.into_iter().map(|course| async move {
            let materials = self.get_materials_by_course_id(course.id).await?;
            Ok((course.id, materials)) as Result<(u64, Vec<Material>)>
        });

        // 并发执行所有任务
        let results = join_all(futures).await;

        // 把结果存入 MATERIALS 中
        for res in results {
            let (course_id, materials) = res?;
            MATERIALS.lock().unwrap().insert(course_id, materials);
        }
        Ok(())
    }

    pub async fn fetch_upload(
        &self,
        reference_id: u64,
        title: String,
        name: String,
        app: AppHandle,
    ) -> Result<()> {
        let url = format!("https://courses.zju.edu.cn/api/uploads/reference/{reference_id}/blob");
        let res = self.client.get(url).send().await?;
        let path = CONFIG.read().unwrap().courseware_dir.join(title);
        std::fs::create_dir_all(&path)?;
        let mut file = File::create(path.join(&name)).await?;
        let mut stream = res.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
        }
        {
            let mut record = RECORD.lock().unwrap();
            if !record.contains(&reference_id) {
                record.push(reference_id);
            }
        }
        app.opener()
            .open_path(path.join(name).to_str().unwrap(), None::<&str>)
            .unwrap();
        Ok(())
    }
}

#[tauri::command]
pub fn init_materials(app: AppHandle) -> Result<(), String> {
    let materials = &*MATERIALS.lock().unwrap();
    app.emit("materials-inited", materials)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_materials() -> Result<HashMap<u64, Vec<Material>>, String> {
    SESSION.get_materials().await.map_err(|e| e.to_string())?;
    Ok(MATERIALS.lock().unwrap().clone())
}

lazy_static! {
    pub static ref RECORD: Mutex<Vec<u64>> = Mutex::new(Vec::<u64>::load());
}

impl Store for Vec<u64> {
    fn store(&self) -> Result<(), String> {
        let record_dir = CONFIG_DIR.join("record.json");
        let record_str = serde_json::to_string(self).map_err(|e| e.to_string())?;
        std::fs::write(record_dir, record_str).map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl Load for Vec<u64> {
    fn load() -> Self {
        let record_dir = CONFIG_DIR.join("record.json");
        if record_dir.exists() {
            let Ok(reader) = std::fs::File::open(record_dir) else {
                return Vec::new();
            };
            let Ok(record) = serde_json::from_reader(reader) else {
                return Vec::new();
            };
            record
        } else {
            Vec::new()
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn fetch_upload(
    app: AppHandle,
    reference_id: u64,
    title: String,
    name: String,
) -> Result<(), String> {
    {
        SESSION
            .fetch_upload(reference_id, title, name, app)
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
