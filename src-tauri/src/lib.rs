extern crate log;
extern crate prost;
extern crate serde;
extern crate serde_json;
extern crate tauri;
extern crate tauri_plugin_shell;
extern crate tokio;
extern crate tonic;

mod commands;
mod proto;

use std::path::PathBuf;

use log::LevelFilter;
use skuld::log::SkuldLogger;
use tauri::Manager;

#[cfg_attr(
    any(target_os = "android", target_os = "ios"),
    tauri::mobile_entry_point
)]
pub fn run() {
    #[cfg(debug_assertions)]
    SkuldLogger::new(PathBuf::from("skuld.log"))
        .unwrap()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::auth_login,
            commands::auth_create_user,
            commands::jobs_list,
            commands::jobs_get,
            commands::jobs_post,
            commands::jobs_claim,
            commands::jobs_pickup,
            commands::jobs_dropoff,
            commands::jobs_cancel,
            commands::jobs_unclaim,
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
