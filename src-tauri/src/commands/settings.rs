use std::sync::Mutex;

use pickledb::PickleDb;

use crate::types::{constants::DB_KEY_SETTINGS, structs::Settings};

#[tauri::command]
pub fn save_settings(settings: Settings, db: tauri::State<'_, Mutex<PickleDb>>) {
    let mut new_db = db.lock().unwrap();
    new_db.set(DB_KEY_SETTINGS, &settings).unwrap();
    new_db.get(DB_KEY_SETTINGS).unwrap()
}

#[tauri::command]
pub fn get_settings(db: tauri::State<'_, Mutex<PickleDb>>) -> Settings {
    let new_db = db.lock().unwrap();
    new_db.get(DB_KEY_SETTINGS).unwrap()
}
