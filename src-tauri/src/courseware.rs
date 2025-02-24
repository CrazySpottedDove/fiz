use crate::{
    session::{Session, SESSION},
    utils::{Dir, Load, Store, CONFIG_DIR},
};
use anyhow::Result;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};
const SEMESTERS_URL: &str = "https://courses.zju.edu.cn/api/my-semesters?";
const COURSES_URL: &str = "https://courses.zju.edu.cn/api/my-courses?conditions=%7B%22status%22:%5B%22ongoing%22,%22notStarted%22%5D,%22keyword%22:%22%22,%22classify_type%22:%22recently_started%22,%22display_studio_list%22:false%7D&fields=id,name,semester_id,course_attributes&page=1&page_size=1000";
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
        let year = self.name[0..9].to_string();
        let term = self.name[9..].to_string();
        (year, term)
    }
}

lazy_static! {
    pub static ref SEMESTERS: Mutex<Vec<Semester>> = Mutex::new(Vec::new());
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
                    year = name[0..9].to_string();
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Course {
    pub id: u64,
    pub name: String,
    pub is_active: bool,
    pub semester_id: u64,
    pub time: String,
}

impl Course {
    pub fn new(id: u64, name: String, is_active: bool, semester_id: u64, time: String) -> Self {
        Self {
            id,
            name,
            is_active,
            semester_id,
            time,
        }
    }
}
impl Dir for Vec<Course> {
    fn dir() -> std::path::PathBuf {
        CONFIG_DIR.join("courses.json")
    }
}
impl Load for Vec<Course> {}
impl Store for Vec<Course> {}

lazy_static! {
    pub static ref COURSES: Mutex<Vec<Course>> = Mutex::new(Vec::<Course>::load());
}

impl Session {
    pub async fn get_courses(&self) -> Result<()> {
        let res = self.client.get(COURSES_URL).send().await?;
        let json = res.json::<Value>().await?;
        let courses_array = json["courses"].as_array().unwrap();
        let courses = courses_array
            .iter()
            .map(|item| {
                let id = item["id"].as_u64().unwrap();
                let name = item["name"].as_str().unwrap().to_string();
                let semester_id = item["semester_id"].as_u64().unwrap();
                let time = &item["course_attributes"]["teaching_class_name"];
                let time = if time.is_null() {
                    String::new()
                } else {
                    time.as_str().unwrap().to_string()
                };
                Course::new(id, name, false, semester_id, time)
            })
            .collect::<Vec<Course>>();
        *COURSES.lock().unwrap() = courses;
        Ok(())
    }
}

#[tauri::command]
pub fn init_courses(app: AppHandle) -> Result<(), String> {
    let courses = &*COURSES.lock().unwrap();
    app.emit("courses-inited", courses)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_semesters() -> Result<(), String> {
    SESSION.get_semesters().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_courses() -> Result<Vec<Course>, String> {
    SESSION.get_courses().await.map_err(|e| e.to_string())?;
    for course in COURSES.lock().unwrap().iter_mut() {
        for semester in SEMESTERS.lock().unwrap().iter() {
            if semester.id == course.semester_id && semester.is_active {
                course.is_active = true;
                break;
            }
        }
    }
    Ok(COURSES.lock().unwrap().clone())
}
