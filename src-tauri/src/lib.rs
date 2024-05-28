#![feature(concat_idents)]

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    SkuldLogger::new(PathBuf::from("skuld.log"))
        .unwrap()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::grpc_login,
            commands::grpc_create_user
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
