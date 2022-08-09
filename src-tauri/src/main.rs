#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod types;

use std::sync::Mutex;

use commands::app_state::{get_app_state, save_app_state};
use commands::db::get_database;
use commands::settings::{get_settings, save_settings};

use tauri_plugin_log::{LogTarget, LoggerBuilder};

fn main() {
    let db = Mutex::new(get_database());

    tauri::Builder::default()
        .plugin(LoggerBuilder::new().targets([LogTarget::Stdout]).build())
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            get_app_state,
            save_app_state,
            get_settings,
            save_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
