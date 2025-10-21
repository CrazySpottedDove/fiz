use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    account::ACCOUNT,
    courseware::SEMESTERS,
    session::{Session, MAX_RETRIES, SESSION},
};
#[derive(Serialize, Deserialize)]
pub struct Test {
    pub name: String,
    pub room: String,
    pub sit: String,
    pub time: String,
}

impl Session {
    pub async fn get_tests(&self) -> Result<Vec<Test>> {
        let id = ACCOUNT.lock().await.stuid.clone();
        // https://zdbk.zju.edu.cn/jwglxt/xskscx/kscx_cxXsgrksIndex.html?doType=query&gnmkdm=N509070&su=3230104178
        let url = format!("https://zdbk.zju.edu.cn/jwglxt/xskscx/kscx_cxXsgrksIndex.html?doType=query&gnmkdm=N509070&layout=default&su={id}#");
        let form = json!(
            {
                "_search": false,
                "nd": &SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
                "queryModel.showCount": 1000,
                "queryModel.currentPage": 1,
                "queryModel.sortName": "xkkh",
                "queryModel.sortOrder": "asc",
                "time": 0
            }
        );
        let active_semesters: Vec<String> = SEMESTERS
            .lock()
            .await
            .iter()
            .filter_map(|semester| {
                // todo: 学期末的时候，学期自动被置为不活跃，但是考试应当是活跃的，需要解决这个问题
                if !semester.is_active {
                    return None;
                }
                let (year, term) = semester.parse_name();
                let id = match term {
                    "春" | "夏" | "春夏" => "2",
                    "秋" | "冬" | "秋冬" | "短" => "1",
                    _ => "",
                };
                if id.is_empty() {
                    return None;
                }
                Some(format!("{year}-{id}"))
            })
            .collect();

        for _ in 1..=MAX_RETRIES {
            let Ok(res) = self.client.post(&url).form(&form).send().await else {
                continue;
            };
            let Ok(json) = res.json::<Value>().await else {
                continue;
            };
            let Some(items) = json["items"].as_array() else {
                continue;
            };

            // let res = match self
            //     .client
            //     .post(&url)
            //     .form(&form)
            //     .send()
            //     .await
            // {
            //     Ok(res) => res,
            //     Err(e) => {
            //         println!("请求发送失败: {:?}", e);
            //         continue;
            //     }
            // };
            // let text = res.text().await;
            // match text {
            //     Ok(ref t) => println!("收到响应: {}", t),
            //     Err(e) => {
            //         println!("读取响应失败: {:?}", e);
            //         continue;
            //     }
            // }
            // let json: serde_json::Result<Value> = serde_json::from_str(text.as_ref().unwrap());
            // let json = match json {
            //     Ok(j) => j,
            //     Err(e) => {
            //         println!("解析JSON失败: {:?}", e);
            //         continue;
            //     }
            // };
            // let Some(items) = json["items"].as_array() else {
            //     println!("items 字段不存在或不是数组: {:?}", json);
            //     continue;
            // };
            let tests: Vec<Test> = items
                .iter()
                .filter_map(|item| {
                    let semester = &item["xkkh"].as_str().unwrap()[1..12];
                    if !active_semesters.contains(&semester.to_string()) {
                        return None;
                    }
                    let time = &item["kssj"];
                    if time.is_null() {
                        return None;
                    }
                    let time = time.as_str().unwrap().to_string();
                    let room = &item["jsmc"];
                    let room = if room.is_null() {
                        String::new()
                    } else {
                        room.as_str().unwrap().to_string()
                    };
                    let name = item["kcmc"].as_str().unwrap().to_string();
                    let sit = &item["zwxh"];
                    let sit = if sit.is_null() {
                        String::new()
                    } else {
                        sit.as_str().unwrap().to_string()
                    };
                    return Some(Test {
                        name,
                        room,
                        sit,
                        time,
                    });
                })
                .collect();
            return Ok(tests);
        }
        Err(anyhow!("获取考试信息失败"))
    }
}

#[tauri::command]
pub async fn get_tests() -> Result<Vec<Test>, String> {
    let tests = SESSION.get_tests().await.map_err(|e| e.to_string())?;
    Ok(tests)
}
