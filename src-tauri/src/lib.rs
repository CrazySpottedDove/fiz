mod account;
mod courseware;
mod grade;
mod session;
mod utils;
use tauri::http::Response;
use utils::ASSETS_DIR;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .register_uri_scheme_protocol("assets", |_, request| {
            // 获取 URI，例如 "dynamic://80058.jpg"
            let uri = request.uri();
            // 去除前缀，得到文件名
            let file_name = uri
                .to_string()
                .replace("assets://", "")
                .trim_end_matches('/')
                .to_string();
            // 拼接实际的文件路径
            println!("{file_name}");
            let file_path = ASSETS_DIR.join(&file_name);
            println!("文件路径: {:?}", file_path);
            if file_path.exists() {
                println!("文件存在");
            } else {
                println!("文件不存在");
            }
            // 尝试读取文件数据
            match std::fs::read(&file_path) {
                Ok(data) => {
                    // 设置响应的 Content-Type，根据文件扩展名判断
                    let content_type = if file_name.ends_with(".png") {
                        "image/png"
                    } else if file_name.ends_with(".jpg") || file_name.ends_with(".jpeg") {
                        "image/jpeg"
                    } else {
                        "application/octet-stream"
                    };
                    Response::builder()
                        .header("Content-Type", content_type)
                        .body(data)
                        .unwrap()
                }
                Err(e) => Response::builder()
                    .status(404)
                    .body(e.to_string().into_bytes())
                    .unwrap(),
            }
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                if let Err(e) = utils::store() {
                    eprintln!("保存数据失败: {}", e);
                }
                window.close().unwrap();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            account::check_account,
            utils::check_dir,
            account::login,
            account::relogin,
            courseware::init_semesters,
            courseware::init_courses,
            courseware::get_courses,
            grade::init_grades_and_analysis,
            grade::get_grades_and_analysis,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
