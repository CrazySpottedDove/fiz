use crate::{
    session::{Session, SESSION},
    utils::{Load, Store, ASSETS_DIR, CONFIG_DIR},
};
use anyhow::Result;
use futures::StreamExt;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs, io::Write, path::PathBuf, sync::Mutex};
use tauri::{AppHandle, Emitter};
const SEMESTERS_URL: &str = "https://courses.zju.edu.cn/api/my-semesters?";
const COURSES_URL: &str = "https://courses.zju.edu.cn/api/my-courses?conditions=%7B%22status%22:%5B%22ongoing%22,%22notStarted%22%5D,%22keyword%22:%22%22,%22classify_type%22:%22recently_started%22,%22display_studio_list%22:false%7D&fields=id,name,semester_id,small_cover&page=1&page_size=1000";
#[derive(Serialize, Deserialize, Clone)]
pub struct Semester {
    pub id: u64,
    pub name: String,
    pub is_active: bool,
}
impl Semester {
    pub fn new(id: u64, name: String, is_active: bool) -> Self {
        Self {
            id,
            name,
            is_active,
        }
    }
    pub fn parse_name(&self) -> (String, String) {
        let year = self.name[0..4].to_string();
        let term = self.name[9..].to_string();
        (year, term)
    }
}

impl Load for Vec<Semester>{
    fn load() -> Self {
        let semesters_dir = CONFIG_DIR.join("semesters.json");
        if semesters_dir.exists() {
            let Ok(reader) = std::fs::File::open(semesters_dir) else {
                return Self::new();
            };
            let Ok(semesters) = serde_json::from_reader(reader) else {
                return Self::new();
            };
            semesters
        } else {
            Self::new()
        }
    }
}

lazy_static! {
    pub static ref SEMESTERS: Mutex<Vec<Semester>> = Mutex::new(Vec::<Semester>::load());
}

impl Session {
    pub async fn get_semesters(&self) -> Result<()> {
        let semesters_res = self.client.get(SEMESTERS_URL).send().await?;
        let json = semesters_res.json::<Value>().await?;
        let mut year = String::new();
        let mut term = String::new();
        let mut semesters = json["semesters"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| {
                let is_active = item["is_active"].as_bool().unwrap();
                let id = item["id"].as_u64().unwrap();
                let name = item["name"].as_str().unwrap().to_string();
                if is_active && ["春夏", "秋冬", "短"].contains(&&name[9..]) {
                    year = name[0..4].to_string();
                    term = name[9..].to_string();
                }
                Semester::new(id, name, is_active)
            })
            .collect::<Vec<Semester>>();
        let more_term = match term.as_str() {
            "春夏" => vec!["春", "夏"],
            "秋冬" => vec!["秋", "冬"],
            _ => vec![""],
        };
        if !more_term.is_empty() {
            for semester in &mut semesters {
                let (this_year, this_term) = semester.parse_name();
                if this_year == year && more_term.contains(&&this_term[..]) {
                    semester.is_active = true;
                }
            }
        }
        *SEMESTERS.lock().unwrap() = semesters;
        Ok(())
    }
}

impl Store for Vec<Semester> {
    fn store(&self) -> Result<(), String> {
        let semesters_dir = CONFIG_DIR.join("semesters.json");
        let semesters_str = serde_json::to_string(self).map_err(|e| e.to_string())?;
        std::fs::write(semesters_dir, semesters_str).map_err(|e| e.to_string())?;
        Ok(())
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Course {
    pub id: u64,
    pub name: String,
    pub is_active: bool,
    pub semester_id: u64,
    pub cover: bool,
}

impl Course {
    pub fn new(id: u64, name: String, is_active: bool, semester_id: u64, cover:bool) -> Self {
        Self {
            id,
            name,
            is_active,
            semester_id,
            cover,
        }
    }
}

impl Load for Vec<Course>{
    fn load() -> Self {
        let courses_dir = CONFIG_DIR.join("courses.json");
        if courses_dir.exists() {
            let Ok(reader) = std::fs::File::open(courses_dir) else {
                return Self::new();
            };
            let Ok(courses) = serde_json::from_reader(reader) else {
                return Self::new();
            };
            courses
        } else {
            Self::new()
        }
    }
}
impl Store for Vec<Course> {
    fn store(&self) -> Result<(), String> {
        let courses_dir = CONFIG_DIR.join("courses.json");
        let courses_str = serde_json::to_string(self).map_err(|e| e.to_string())?;
        std::fs::write(courses_dir, courses_str).map_err(|e| e.to_string())?;
        Ok(())
    }
}


lazy_static! {
    pub static ref COURSES: Mutex<Vec<Course>> = Mutex::new(Vec::<Course>::load());
}

impl Session {
    pub async fn download_cover(&self, url: &str, id: String) -> Result<()> {
        let res = self.client.get(url).send().await?;
        let content_type = res
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");
        let extension = match content_type {
            "image/jpeg" => "jpg",
            "image/png" => "png",
            _ => "jpg",
        };
        let cover_dir = ASSETS_DIR.join(format!("{id}.{extension}"));
        let mut stream = res.bytes_stream();
        let mut file = fs::File::create(cover_dir)?;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk)?;
        }
        Ok(())
    }
    pub async fn get_courses(&self) -> Result<()> {
        let res = self.client.get(COURSES_URL).send().await?;
        let json = res.json::<Value>().await?;
        let ids = COURSES
            .lock()
            .unwrap()
            .iter()
            .map(|course| course.id)
            .collect::<Vec<u64>>();
        let courses_array = json["courses"].as_array().unwrap();

        // 并发下载缺失的封面图片
        use futures::stream::{FuturesUnordered, StreamExt};
        let mut downloads = FuturesUnordered::new();
        for item in courses_array {
            let id = item["id"].as_u64().unwrap();
            if !ids.contains(&id) {
                let small_cover = item["small_cover"].as_str().unwrap();
                if small_cover.is_empty() {
                    continue;
                }
                let download = self.download_cover(small_cover, id.to_string());
                // 注意这里直接传入self.download_cover返回的Future
                downloads.push(download);
            }
        }
        // 并发执行所有下载任务
        while let Some(result) = downloads.next().await {
            result?;
        }

        // 构造 Course 对象
        let courses = courses_array
            .iter()
            .map(|item| {
                let id = item["id"].as_u64().unwrap();
                let name = item["name"].as_str().unwrap().to_string();
                let semester_id = item["semester_id"].as_u64().unwrap();
                let is_active = !item["credit_state"].is_null();
                let cover = !item["small_cover"].as_str().unwrap().is_empty();
                Course::new(id, name, is_active, semester_id,cover)
            })
            .collect::<Vec<Course>>();
        *COURSES.lock().unwrap() = courses;
        Ok(())
    }
}

#[tauri::command]
pub fn init_semesters(app: AppHandle) -> Result<(), String> {
    let semesters = &*SEMESTERS.lock().unwrap();
    app.emit("semesters-inited", semesters)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn init_courses(app: AppHandle) -> Result<(), String> {
    let courses = &*COURSES.lock().unwrap();
    app.emit("courses-inited", courses)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_courses() -> Result<Vec<Course>, String> {
    SESSION.get_courses().await.map_err(|e| e.to_string())?;
    Ok(COURSES.lock().unwrap().clone())
}


