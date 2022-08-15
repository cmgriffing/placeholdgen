#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod types;

use commands::images::{add_local_image, add_remote_image, get_all_images};

use tauri_plugin_log::{LogTarget, LoggerBuilder};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sqlite::init())
        .plugin(LoggerBuilder::new().targets([LogTarget::Stdout]).build())
        .invoke_handler(tauri::generate_handler![
            add_local_image,
            add_remote_image,
            get_all_images,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
