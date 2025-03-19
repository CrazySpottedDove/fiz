use crate::{
    courseware::COURSES,
    session::{Session, MAX_RETRIES, SESSION},
    utils::{Dir, Load, Store, CONFIG, CONFIG_DIR},
};
use anyhow::{anyhow, Result};
use futures::{future::join_all, StreamExt};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
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
impl Dir for HashMap<u64, Vec<Material>> {
    fn dir() -> std::path::PathBuf {
        CONFIG_DIR.join("materials.json")
    }
}
impl Load for HashMap<u64, Vec<Material>> {}

impl Store for HashMap<u64, Vec<Material>> {}

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
        let url = format!("https://courses.zju.edu.cn/api/course/{id}/coursewares?conditions=%7B%22category%22:null,%22itemsSortBy%22:%7B%22predicate%22:%22chapter%22,%22reverse%22:false%7D,%22ignore_activity_types%22:%5B%22lesson%22%5D%7D&page=1&page_size=1000");
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
        Err(anyhow!("获取课件失败：超过最大尝试次数"))
    }
    pub async fn get_materials(&self) -> Result<()> {
        // 克隆一下 courses，避免持有锁太久
        let courses = COURSES.lock().await.clone();
        let all = !CONFIG.read().unwrap().less;
        // 为每个课程创建一个异步任务，获取活动信息
        let futures =
            courses
                .into_iter()
                .filter(|course| all | course.is_active)
                .map(|course| async move {
                    let materials = self.get_materials_by_course_id(course.id).await?;
                    Ok((course.id, materials)) as Result<(u64, Vec<Material>)>
                });

        // 并发执行所有任务
        let results = join_all(futures).await;

        // 把结果存入 MATERIALS 中
        for res in results {
            let (course_id, materials) = res?;
            MATERIALS.lock().await.insert(course_id, materials);
        }
        Ok(())
    }

    pub async fn fetch_upload(&self, id: u64, title: &str, name: &str) -> Result<()> {
        let is_office_file = name.ends_with(".docx")
            || name.ends_with(".doc")
            || name.ends_with(".pptx")
            || name.ends_with(".ppt")
            || name.ends_with(".xlsx")
            || name.ends_with(".xls");
        let pdf = CONFIG.read().unwrap().pdf;
        let url = if pdf && is_office_file {
            format!("https://courses.zju.edu.cn/api/uploads/document/{id}/url?preview=true")
        } else {
            format!("https://courses.zju.edu.cn/api/uploads/{id}/blob")
        };
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
            let mut record = RECORD.lock().await;
            if !record.contains(&id) {
                record.push(id);
            }
        }
        Ok(())
    }
}

#[tauri::command]
pub async fn init_materials(app: AppHandle) -> Result<(), String> {
    let materials = &*MATERIALS.lock().await;
    app.emit("materials-inited", materials)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_materials() -> Result<HashMap<u64, Vec<Material>>, String> {
    SESSION.get_materials().await.map_err(|e| e.to_string())?;
    Ok(MATERIALS.lock().await.clone())
}

lazy_static! {
    pub static ref RECORD: Mutex<Vec<u64>> = Mutex::new(Vec::<u64>::load());
}

impl Store for Vec<u64> {}
impl Dir for Vec<u64> {
    fn dir() -> std::path::PathBuf {
        CONFIG_DIR.join("record.json")
    }
}
impl Load for Vec<u64> {}

#[tauri::command(rename_all = "snake_case")]
pub async fn fetch_upload(id: u64, title: String, name: String) -> Result<(), String> {
    {
        SESSION
            .fetch_upload(id, &title, &name)
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
