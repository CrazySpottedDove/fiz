use anyhow::{anyhow, Result};
use chrono::DateTime;
use futures::future::join_all;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};

use crate::courseware::COURSES;
use crate::session::{Session, MAX_RETRIES, SESSION};
use crate::utils::Load;
use crate::utils::CONFIG_DIR;
use crate::{material::Upload, utils::Store};
#[derive(Serialize, Deserialize, Clone)]
pub struct Homework {
    pub id: u64,
    pub title: String,
    pub uploads: Vec<Upload>,
    pub course: String,
    pub ddl: String,
    pub submitted: bool,
    pub description: String,
}

impl Homework {
    pub fn new(
        id: u64,
        title: String,
        uploads: Vec<Upload>,
        course: String,
        ddl: String,
        submitted: bool,
        description: String,
    ) -> Self {
        Self {
            id,
            title,
            uploads,
            course,
            ddl,
            submitted,
            description,
        }
    }
}

impl Load for Vec<Homework> {
    fn load() -> Self {
        let homeworks_dir = CONFIG_DIR.join("homeworks.json");
        if homeworks_dir.exists() {
            let Ok(reader) = std::fs::File::open(homeworks_dir) else {
                return Vec::new();
            };
            let Ok(homeworks) = serde_json::from_reader(reader) else {
                return Vec::new();
            };
            homeworks
        } else {
            Vec::new()
        }
    }
}

impl Store for Vec<Homework> {
    fn store(&self) -> Result<(), String> {
        let homeworks_dir = CONFIG_DIR.join("homeworks.json");
        let homeworks_str = serde_json::to_string(self).map_err(|e| e.to_string())?;
        std::fs::write(homeworks_dir, homeworks_str).map_err(|e| e.to_string())?;
        Ok(())
    }
}

lazy_static! {
    pub static ref HOMEWORKS: Mutex<Vec<Homework>> = Mutex::new(Vec::<Homework>::load());
}

impl Session {
    pub async fn get_homework(&self, id: u64, course: String) -> Result<Vec<Homework>> {
        let url = format!(
            "https://courses.zju.edu.cn/api/courses/{id}/homework-activities?page=1&page_size=1000"
        );
        for _ in 1..MAX_RETRIES {
            let Ok(res) = self.client.get(&url).send().await else {
                continue;
            };
            let Ok(json) = res.json::<serde_json::Value>().await else {
                continue;
            };
            let homeworks_json = json["homework_activities"]
                .as_array()
                .ok_or(anyhow!("Invalid json"))?;
            let homeworks = homeworks_json
                .iter()
                .filter_map(|homework| {
                    if homework["is_closed"].as_bool().unwrap() {
                        return None;
                    }
                    let description = homework["data"]["description"]
                        .as_str()
                        .unwrap_or("")
                        .to_string();
                    let submitted = homework["submitted"].as_bool()?;
                    let id = homework["id"].as_u64()?;
                    let title = homework["title"].as_str()?.to_string();
                    let course = course.clone();
                    let ddl_str = homework["deadline"]
                        .as_str()
                        ?;
                    let ddl = DateTime::parse_from_rfc3339(ddl_str).ok()?
                        .format("%Y-%m-%d %H:%M")
                        .to_string();
                    let uploads = homework["uploads"]
                        .as_array()
                        ?;
                    let mut upload_vec = Vec::new();
                    for upload in uploads {
                        let upload_id = upload["id"].as_u64()?;
                        let name = upload["name"]
                            .as_str()
                            ?
                            .to_string();
                        let reference_id = upload["reference_id"]
                            .as_u64()
                            ?;
                        upload_vec.push(Upload::new(upload_id, reference_id, name));
                    }
                    Some(Homework::new(
                        id,
                        title,
                        upload_vec,
                        course,
                        ddl,
                        submitted,
                        description,
                    ))
                })
                .collect::<Vec<Homework>>();
            return Ok(homeworks);
        }
        Err(anyhow!("Failed to get homework"))
    }
    pub async fn get_homeworks(&self) -> Result<()> {
        let courses = COURSES.lock().unwrap().clone();
        let futures = courses.into_iter().map(|course| async move {
            let homeworks = self.get_homework(course.id, course.name).await?;
            Ok(homeworks) as Result<Vec<Homework>>
        });

        let results = join_all(futures).await;
        let mut all_homeworks = Vec::new();
        for result in results {
            match result {
                Ok(homeworks) => all_homeworks.extend(homeworks),
                Err(e) => eprintln!("获取作业失败: {}", e),
            }
        }
        *HOMEWORKS.lock().unwrap() = all_homeworks;
        Ok(())
    }
}

#[tauri::command]
pub async fn get_homeworks() -> Result<Vec<Homework>, String> {
    SESSION.get_homeworks().await.map_err(|e| e.to_string())?;
    Ok(HOMEWORKS.lock().unwrap().clone())
}

#[tauri::command]
pub fn init_homeworks(app: AppHandle) -> Result<(), String> {
    let homeworks = &*HOMEWORKS.lock().unwrap();
    app.emit("homeworks_inited", homeworks)
        .map_err(|e| e.to_string())?;
    Ok(())
}
