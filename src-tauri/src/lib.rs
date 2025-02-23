mod account;
mod courseware;
mod grade;
mod homework;
mod material;
mod preview;
mod session;
mod test;
mod utils;
mod watch;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
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
            courseware::init_courses,
            courseware::get_courses,
            grade::init_grades_and_analysis,
            grade::get_grades_and_analysis,
            material::init_materials,
            material::get_materials,
            preview::get_preview,
            utils::init_config,
            utils::update_config,
            material::fetch_upload,
            homework::init_homeworks,
            homework::get_homeworks,
            homework::submit_homework,
            homework::submit_file,
            homework::query_file,
            test::get_tests,
            courseware::get_semesters,
            watch::get_watches,
            watch::init_watches,
            watch::set_watches
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
