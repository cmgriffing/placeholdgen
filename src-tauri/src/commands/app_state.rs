use std::sync::Mutex;

use pickledb::PickleDb;

use crate::types::{constants::DB_KEY_APP_STATE, structs::AppState};

#[tauri::command]
pub fn save_app_state(app_state: AppState, db: tauri::State<'_, Mutex<PickleDb>>) -> AppState {
    println!("AppState: {:#?}", app_state);

    let mut new_db = db.lock().unwrap();

    new_db.set(DB_KEY_APP_STATE, &app_state).unwrap();

    match new_db.get::<AppState>(DB_KEY_APP_STATE) {
        Some(new_app_state) => new_app_state,
        None => {
            println!("Error: None found when fetching");
            app_state
        }
    }
}

#[tauri::command]
pub fn get_app_state(db: tauri::State<'_, Mutex<PickleDb>>) -> AppState {
    let new_db = db.lock().unwrap();
    let app_state = match new_db.get::<AppState>(DB_KEY_APP_STATE) {
        Some(new_app_state) => {
            println!("found value: {:#?}", new_app_state);
            new_app_state
        }
        None => {
            println!("Error: None found when fetching");
            AppState { sites: vec![] }
        }
    };

    println!("AppState: {:#?}", app_state);

    app_state
}
