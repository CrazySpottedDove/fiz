use anyhow::{anyhow, Result};
use chrono::DateTime;
use futures::channel::oneshot;
use futures::future::join_all;
use lazy_static::lazy_static;
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;
use tokio::sync::Mutex;
use tauri::{AppHandle, Emitter};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
const SUBMIT_URL: &str = "https://courses.zju.edu.cn/api/uploads";

use crate::courseware::COURSES;
use crate::session::{Session, MAX_RETRIES, SESSION};
use crate::utils::CONFIG_DIR;
use crate::utils::{Dir, Load};
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
impl Dir for Vec<Homework> {
    fn dir() -> PathBuf {
        CONFIG_DIR.join("homeworks.json")
    }
}
impl Load for Vec<Homework> {}
impl Store for Vec<Homework> {}

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
                    let ddl_str = homework["deadline"].as_str()?;
                    let ddl = DateTime::parse_from_rfc3339(ddl_str)
                        .ok()?
                        .format("%Y-%m-%d %H:%M")
                        .to_string();
                    let uploads = homework["uploads"].as_array()?;
                    let mut upload_vec = Vec::new();
                    for upload in uploads {
                        let upload_id = upload["id"].as_u64()?;
                        let name = upload["name"].as_str()?.to_string();
                        let reference_id = upload["reference_id"].as_u64()?;
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
        let courses = COURSES.lock().await.clone();
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
        *HOMEWORKS.lock().await = all_homeworks;
        Ok(())
    }
}

#[tauri::command]
pub async fn get_homeworks() -> Result<Vec<Homework>, String> {
    SESSION.get_homeworks().await.map_err(|e| e.to_string())?;
    Ok(HOMEWORKS.lock().await.clone())
}

#[tauri::command]
pub async fn init_homeworks(app: AppHandle) -> Result<(), String> {
    let homeworks = &*HOMEWORKS.lock().await;
    app.emit("homeworks-inited", homeworks)
        .map_err(|e| e.to_string())?;
    Ok(())
}

impl Session {
    pub async fn submit_homework(
        &self,
        homework_id: u64,
        file_id: u64,
        mut comment: String,
    ) -> Result<()> {
        let handin_url =
            format!("https://courses.zju.edu.cn/api/course/activities/{homework_id}/submissions");

        if !comment.is_empty() {
            comment = format!("<p>{comment}<br></p>");
        }
        let payload = json!({
            "comment":comment,
            "is_draft":false,
            "mode":"normal",
            "other_resources":[],
            "slides":[],
            "uploads":[file_id],
            "uploads_in_rich_text":[]
        });

        let mut json: Option<Value> = None; // 使用 Option 包装

        for _ in 1..=MAX_RETRIES {
            let Ok(res) = self.client.post(&handin_url).json(&payload).send().await else {
                continue;
            };
            if let Ok(json_unjudged) = res.json::<Value>().await {
                #[cfg(debug_assertions)]
                println!("SUBMIT POST response as JSON: {:#?}", json_unjudged);
                if json_unjudged["errors"].is_array() {
                    return Err(anyhow!("上交作业失败"));
                }
                json = Some(json_unjudged);
                break;
            } else {
                continue;
            }
        }

        if json.is_none() {
            return Err(anyhow!("上传作业失败"));
        }
        Ok(())
    }
    pub async fn submit_file(&self, file_path: PathBuf) -> Result<u64> {
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        let file_size = file_path.metadata().unwrap().len();
        let payload = json!({
            "embed_material_type": "",
            "is_marked_attachment": false,
            "is_scorm": false,
            "is_wmpkg": false,
            "name": file_name,
            "parent_id": 0,
            "parent_type": null,
            "size": file_size,
            "source": ""
        });
        let mut json: Option<Value> = None;
        for _ in 1..=MAX_RETRIES {
            let Ok(res) = self.client.post(SUBMIT_URL).json(&payload).send().await else {
                continue;
            };
            let Ok(json_unjudged) = res.json::<Value>().await else {
                continue;
            };
            let err = &json_unjudged["errors"];
            if err.is_object() {
                return Err(anyhow!("上传失败: {err}"));
            }
            json = Some(json_unjudged);
            break;
        }
        let Some(json) = json else {
            return Err(anyhow!("上传请求失败"));
        };
        let Some(upload_url) = json["upload_url"].as_str() else {
            return Err(anyhow!("上传请求返回无 upload_url 字段"));
        };
        let Some(id) = json["id"].as_u64() else {
            return Err(anyhow!("上传请求返回无 id 字段"));
        };
        let Some(file_name) = json["name"].as_str() else {
            return Err(anyhow!("上传请求返回无 name 字段"));
        };
        let mut file = File::open(file_path).await?;
        let mut file_content = Vec::new();
        file.read_to_end(&mut file_content).await?;
        let file_part = multipart::Part::bytes(file_content)
            .file_name(file_name.to_string())
            .mime_str("application/octet-stream")?;
        let form = multipart::Form::new().part("file", file_part);

        let res = self.client.put(upload_url).multipart(form).send().await?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(anyhow!("上传状态码：{status}，响应内容：{text}"));
        }
        Ok(id)
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn submit_homework(
    homework_id: u64,
    file_id: u64,
    comment: String,
) -> Result<(), String> {
    SESSION
        .submit_homework(homework_id, file_id, comment)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn submit_file(file_path: PathBuf) -> Result<u64, String> {
    let file_id = SESSION
        .submit_file(file_path)
        .await
        .map_err(|e| e.to_string())?;
    Ok(file_id)
}

#[tauri::command]
pub async fn query_file(app: AppHandle) -> Result<Option<PathBuf>, String> {
    const EXPECTED_FILE_TYPES: &[&str] = &[
        "avi", "flv", "m4v", "mov", "mp4", "3gp", "3gpp", "mpg", "rm", "rmvb", "swf", "webm",
        "wmv", "mp3", "m4a", "wav", "wma", "jpeg", "jpg", "png", "gif", "bmp", "heic", "webp",
        "txt", "pdf", "csv", "xls", "xlsx", "doc", "ppt", "pptx", "docx", "odp", "ods", "odt",
        "rtf", "zip", "rar", "tar", "mat", "dwg", "m", "mlapp", "slx", "mlx",
    ];
    use tauri_plugin_dialog::DialogExt;
    let (sender, receiver) = oneshot::channel();
    app.dialog()
        .file()
        .add_filter("仅显示学在浙大允许上传的文件", EXPECTED_FILE_TYPES)
        .pick_file(|f| {
            sender.send(f).unwrap();
        });
    let Some(file) = receiver.await.map_err(|e| e.to_string())? else {
        return Ok(None);
    };
    Ok(Some(PathBuf::from(file.to_string())))
}
