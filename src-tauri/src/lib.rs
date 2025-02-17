mod account;
mod courseware;
mod grade;
mod session;
mod utils;
use tauri::http::Response;
use utils::ASSETS_DIR;
use std::fs::OpenOptions;
use std::io::Write;
fn log_debug(message: &str) {
    // 日志文件路径：ASSETS_DIR/debug.log
    let debug_log_path = ASSETS_DIR.join("debug.log");
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(debug_log_path)
    {
        let _ = writeln!(file, "{}", message);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .register_uri_scheme_protocol("assets", |_, request| {
            use percent_encoding::percent_decode_str;
            use std::path::MAIN_SEPARATOR;

            let uri = request.uri();
            log_debug(&format!("Received URI: {}", uri));

            // 解码 host
            let host = uri
                .host()
                .map(|h| {
                    let decoded = percent_decode_str(h).decode_utf8_lossy().to_string();
                    log_debug(&format!("Decoded host: {}", decoded));
                    decoded
                })
                .unwrap_or_default();

            // 解码 path
            let raw_path = percent_decode_str(uri.path())
                .decode_utf8_lossy()
                .to_string();
            log_debug(&format!("Raw path: {}", raw_path));
            let path = raw_path.trim_start_matches('/').to_string();
            log_debug(&format!("Trimmed path: {}", path));

            // 组合逻辑：优先选择 path（如果非空），否则使用 host
            let file_name = if path.is_empty() {
                log_debug("Path empty, using host as file name");
                host
            } else {
                log_debug("Using path as file name");
                path
            };

            // 替换路径分隔符（将 `/` 替换为系统分隔符）
            let file_name = file_name.replace('/', &MAIN_SEPARATOR.to_string());
            log_debug(&format!("File name after separator conversion: {}", file_name));

            // 最终文件路径
            let file_path = ASSETS_DIR.join(&file_name);
            log_debug(&format!("Final file path: {:?}", file_path));

            // 尝试读取文件数据并记录结果
            match std::fs::read(&file_path) {
                Ok(data) => {
                    log_debug("File read successfully");
                    // 根据文件扩展名设置 Content-Type
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
                Err(e) => {
                    log_debug(&format!("Failed to read file: {:?}", e));
                    Response::builder()
                        .status(404)
                        .body(e.to_string().into_bytes())
                        .unwrap()
                }
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
