use crate::commands::db::get_database;
use crate::types::{constants::DB_KEY_SETTINGS, structs::Settings};

#[tauri::command]
pub fn save_settings(settings: Settings) {
    let mut db = get_database();
    db.set(DB_KEY_SETTINGS, &settings).unwrap();
    db.get(DB_KEY_SETTINGS).unwrap()
}

#[tauri::command]
pub fn get_settings() -> Settings {
    let db = get_database();
    db.get(DB_KEY_SETTINGS).unwrap()
}
