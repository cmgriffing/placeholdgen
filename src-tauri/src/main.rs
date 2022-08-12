#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod types;

use std::sync::Mutex;

use commands::app_state::{get_app_state, save_app_state};
use commands::db::{get_database, Database};
use commands::images::add_local_image;
use commands::settings::{get_settings, save_settings};

use pickledb::PickleDb;
use tauri_plugin_log::{LogTarget, LoggerBuilder};

fn main() {
    let main_app = tauri::Builder::default();

    let database = Mutex::new(Database {
        db: Mutex::new(get_database(None)),
    });

    let foo = main_app
        .plugin(LoggerBuilder::new().targets([LogTarget::Stdout]).build())
        // .manage(db)
        .manage(database)
        .invoke_handler(tauri::generate_handler![
            get_app_state,
            save_app_state,
            get_settings,
            save_settings,
            add_local_image
        ]);
    foo.run(tauri::generate_context!())
        .expect("error while running tauri application");

    // let db = Mutex::new(get_database(main_app));
}
