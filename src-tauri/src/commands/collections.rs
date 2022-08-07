use crate::types::{
    constants::DB_KEY_COLLECTIONS,
    structs::{Collection, Settings},
};

use crate::commands::db::get_database;

#[tauri::command]
pub fn save_collections(collections: Vec<Collection>) {
    let mut db = get_database();
    db.set(DB_KEY_COLLECTIONS, &collections).unwrap();
    db.get(DB_KEY_COLLECTIONS).unwrap()
}

#[tauri::command]
pub fn get_collections() {
    let db = get_database();
    db.get(DB_KEY_COLLECTIONS).unwrap()
}
