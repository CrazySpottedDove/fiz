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
            use percent_encoding::percent_decode_str;
            use std::path::MAIN_SEPARATOR;

            let uri = request.uri();

            // 解码 host 和 path
            let host = uri
                .host()
                .map(|h| percent_decode_str(h).decode_utf8_lossy().to_string())
                .unwrap_or_default();
            let path = percent_decode_str(uri.path())
                .decode_utf8_lossy()
                .to_string()
                .trim_start_matches('/')
                .to_string();

            // 组合逻辑：优先使用 host（如果 path 为空）
            let file_name = if path.is_empty() { host } else { path };

            // 转换路径分隔符（将 `/` 替换为系统分隔符）
            let file_name = file_name.replace('/', &MAIN_SEPARATOR.to_string());

            // 最终文件路径
            let file_path = ASSETS_DIR.join(&file_name);

            println!("请求文件: {:?}", file_path);
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
