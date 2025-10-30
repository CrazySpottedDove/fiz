// 负责学在浙大中测试任务的显示

use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use futures::future::join_all;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use std::path::PathBuf;
use tokio::sync::Mutex;

use crate::{
    courseware::COURSES, session::{MAX_RETRIES, SESSION, Session}, utils::{CONFIG, CONFIG_DIR, Dir, Load, Store}
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Quiz {
    // 测试的标题
    pub title: String,
    // 测试的 id
    pub id: u64,
    // 测试是否已提交
    pub submitted: bool,
    // 测试的截止时间
    pub ddl: String,
    // 属于哪门课程
    pub course: String,
}

impl Quiz {
    pub fn new(title: String, id: u64, submitted: bool, ddl: String, course: String) -> Self {
        Self {
            title,
            id,
            submitted,
            ddl,
            course,
        }
    }
}

impl Dir for Vec<Quiz> {
    fn dir() -> PathBuf {
        CONFIG_DIR.join("quizes.json")
    }
}

impl Load for Vec<Quiz> {}
impl Store for Vec<Quiz> {}

lazy_static! {
    pub static ref QUIZES: Mutex<Vec<Quiz>> = Mutex::new(Vec::<Quiz>::load());
}

impl Session {
    // 获取一个课程的测试
    pub async fn get_quiz(&self, id: u64, course: String) -> Result<Vec<Quiz>> {
        let url =
            format!("https://courses.zju.edu.cn/api/courses/{id}/exam-list?page=1&page_size=100");
        for _ in 1..MAX_RETRIES {
            let Ok(res) = self.client.get(&url).send().await else {
                continue;
            };
            let Ok(json) = res.json::<serde_json::Value>().await else {
                continue;
            };
            let quizes_json = json["exams"].as_array().ok_or(anyhow!("Invalid json"))?;
            let quizes = quizes_json
                .iter()
                .filter_map(|quiz| {
                    if quiz["is_closed"].as_bool().unwrap() {
                        return None;
                    }
                    let submission_count = quiz["submission_count"].as_u64()?;
                    let id = quiz["id"].as_u64()?;
                    let title = quiz["title"].as_str()?.to_string();
                    let course = course.clone();
                    let ddl_str = quiz["end_time"].as_str()?;
                    let ddl = DateTime::parse_from_rfc3339(ddl_str)
                        .ok()?
                        .with_timezone(&Local)
                        .format("%Y-%m-%d %H:%M")
                        .to_string();
                    Some(Quiz::new(title, id, submission_count > 0, ddl, course))
                })
                .collect::<Vec<Quiz>>();
            return Ok(quizes);
        }
        Err(anyhow!("Failed to get quiz"))
    }

    // 获得所有课程的测试
    pub async fn get_quizes(&self)->Result<()>{
        let courses = COURSES.lock().await.clone();
        let all = !CONFIG.read().unwrap().less;
        let futures = courses.into_iter().filter(|course| all|course.is_active).map(|course| async move{
            let quizes = self.get_quiz(course.id, course.name).await?;
            Ok(quizes) as Result<Vec<Quiz>>
        });
        let results = join_all(futures).await;
        let mut all_quizes = Vec::new();
        for result in results{
            match result{
                Ok(quizes)=> all_quizes.extend(quizes),
                Err(e)=>eprintln!("获取测试失败: {e}")
            }
        }
        let parse_date = |date: &str| {
            DateTime::parse_from_str(&format!("{} +00:00", date), "%Y-%m-%d %H:%M %z")
                .unwrap_or_else(|_| {
                    eprintln!("非法日期格式: {}", date);
                    Local::now().fixed_offset() // 返回默认时间避免崩溃
                })
        };
        all_quizes.sort_by(|a, b| {
            let a_ddl = parse_date(&a.ddl);
            let b_ddl = parse_date(&b.ddl);
            a_ddl.cmp(&b_ddl)
        });
        *QUIZES.lock().await = all_quizes;
        Ok(())
    }
}

#[tauri::command]
pub async fn init_quizes(app: AppHandle)-> Result<(), String>{
    let quizes = &*QUIZES.lock().await;
    app.emit("quizes-inited", quizes).map_err(|e| e.to_string())?;
    Ok(())
}


#[tauri::command]
pub async fn get_quizes(app: AppHandle) -> Result<Vec<Quiz>, String> {
    SESSION.get_quizes().await.map_err(|e| e.to_string())?;
    Ok(QUIZES.lock().await.clone())
}