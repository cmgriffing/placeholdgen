#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod types;

use commands::collections::{get_collections, save_collections};

use commands::settings::{get_settings, save_settings};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_collections,
            save_collections,
            get_settings,
            save_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
