use crate::account::{ACCOUNT, ETA_URL};
use crate::session::SESSION;
use crate::session::{Session, MAX_RETRIES};
use crate::utils::{Dir, Load, Store, CONFIG_DIR};
use anyhow::Result;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};
const ETA_GRADE_URL: &str = "http://eta.zju.edu.cn/zftal-xgxt-web/api/teacher/xshx/getKccjList.zf";
#[derive(Serialize, Deserialize, Clone)]
pub struct Grade {
    pub name: String,
    pub grade: String,
    pub credit: String,
    pub gpa: f64,
    pub xq: String,
    pub xn: String,
}

impl Grade {
    pub fn new(
        name: String,
        grade: String,
        credit: String,
        gpa: f64,
        xq: String,
        xn: String,
    ) -> Self {
        Self {
            name,
            grade,
            credit,
            gpa,
            xq,
            xn,
        }
    }
    pub fn group_by_xq_and_xn(grades: Vec<Grade>) -> HashMap<(String, String), Vec<Grade>> {
        let mut grouped_grades: HashMap<(String, String), Vec<Grade>> = HashMap::new();
        for grade in grades {
            let xq_group = match grade.xq.as_str() {
                "春" | "夏" | "春夏" => "春夏",
                "秋" | "冬" | "秋冬" | "短" => "秋冬",
                _ => "未知",
            };
            let key = (xq_group.to_string(), grade.xn.clone());
            grouped_grades
                .entry(key)
                .or_insert_with(Vec::new)
                .push(grade);
        }
        grouped_grades
    }
    pub fn analize(grades: Vec<Grade>) -> (f64, f64) {
        let mut total_credit = 0.0;
        let mut total_gpa = 0.0;
        for grade in grades {
            let credit = grade.credit.parse::<f64>().unwrap();
            let gpa = grade.gpa;
            total_credit += credit;
            total_gpa += credit * gpa;
        }
        let gpa = total_gpa / total_credit;

        (gpa, total_credit)
    }
    pub fn analize_by_xq_and_xn(grades: Vec<Grade>) -> Vec<Analysis> {
        let mut analysis = Vec::new();
        let mut total_credit = 0.0;
        let mut total_gpa = 0.0;
        let grouped_grades = Grade::group_by_xq_and_xn(grades);
        for (key, grades) in grouped_grades {
            for grade in &grades {
                let credit = grade.credit.parse::<f64>().unwrap();
                let gpa = grade.gpa;
                total_credit += credit;
                total_gpa += credit * gpa;
            }
            let (gpa, credit) = Grade::analize(grades);
            analysis.push(Analysis {
                xn: key.1,
                xq: key.0,
                gpa,
                credit,
            });
        }
        analysis.push(Analysis {
            xn: "全学年".to_string(),
            xq: "全学期".to_string(),
            gpa: total_gpa / total_credit,
            credit: total_credit,
        });
        analysis
    }
}
lazy_static! {
    pub static ref GRADES: Mutex<Vec<Grade>> = Mutex::new(Vec::<Grade>::load());
}
impl Load for Vec<Grade> {}
impl Dir for Vec<Grade> {
    fn dir() -> std::path::PathBuf {
        CONFIG_DIR.join("grades.json")
    }
}
impl Store for Vec<Grade> {}
impl Session {
    pub async fn get_grades(&self) -> Result<()> {
        let stuid = ACCOUNT.lock().unwrap().as_ref().unwrap().stuid.clone();
        let url = format!("{ETA_GRADE_URL}?xh={stuid}&currentPage=1&showCount=200&xn=&xq=&kcmc=&orders=%5B%5D&sfjg=");
        let mut grades = Vec::new();
        for retry in 1..=MAX_RETRIES {
            let res = self.client.get(&url).send().await?;
            let Ok(json) = res.json::<Value>().await else {
                continue;
            };
            match json["data"]["items"].as_array() {
                Some(grades_json) => {
                    for grade_json in grades_json {
                        if !grade_json["BZ"].is_null()
                            && grade_json["BZ"].as_str().unwrap() == "弃修"
                        {
                            continue;
                        }
                        let grade = if grade_json["CJ"].is_null() {
                            continue;
                        } else {
                            grade_json["CJ"].as_u64().unwrap().to_string()
                        };
                        let name = grade_json["KCMC"].as_str().unwrap().to_string();
                        let credit = grade_json["XF"].as_str().unwrap().to_string();
                        let gpa = grade_json["JD"].as_str().unwrap().parse::<f64>().unwrap();
                        let xq = grade_json["XQ"].as_str().unwrap().to_string();
                        let xn = grade_json["XN"].as_str().unwrap().to_string();
                        grades.push(Grade::new(name, grade, credit, gpa, xq, xn));
                    }
                    break;
                }
                None => {
                    if retry == MAX_RETRIES {
                        return Err(anyhow::anyhow!("获取成绩失败"));
                    }
                    self.client.get(ETA_URL).send().await?;
                }
            }
        }
        *GRADES.lock().unwrap() = grades;
        Ok(())
    }
}
#[derive(Serialize, Deserialize)]
pub struct Analysis {
    pub xn: String,
    pub xq: String,
    pub gpa: f64,
    pub credit: f64,
}
impl Dir for Vec<Analysis> {
    fn dir() -> std::path::PathBuf {
        CONFIG_DIR.join("analysis.json")
    }
}
impl Load for Vec<Analysis> {}
impl Store for Vec<Analysis> {}
lazy_static! {
    pub static ref ANALYSIS: Mutex<Vec<Analysis>> = Mutex::new(Vec::<Analysis>::load());
}
#[tauri::command]
pub fn init_grades_and_analysis(app: AppHandle) -> Result<(), String> {
    let grades = &*GRADES.lock().unwrap();
    let analysis = &*ANALYSIS.lock().unwrap();
    app.emit("grades-and-analysis-inited", (grades, analysis))
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_grades_and_analysis() -> Result<(Vec<Grade>, Vec<Analysis>), String> {
    SESSION.get_grades().await.map_err(|e| e.to_string())?;
    let grades = GRADES.lock().unwrap().clone();
    let analysis = Grade::analize_by_xq_and_xn(grades.clone());
    Ok((grades, analysis))
}
