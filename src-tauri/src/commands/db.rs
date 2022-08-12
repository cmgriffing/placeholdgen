// https://github.com/tauri-apps/tauri/blob/dev/examples/state/main.rs#L22-L23

use std::sync::Mutex;

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use tauri::App;

pub fn get_database(app_option: Option<&mut App>) -> PickleDb {
    let base_db_path = match app_option {
        Some(app) => app
            .path_resolver()
            .app_dir()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
            .to_owned(),
        None => ".".to_string(),
    };

    let db_path = format!("{}/{}", base_db_path, "db.db");

    match PickleDb::load(
        db_path.clone(),
        PickleDbDumpPolicy::DumpUponRequest,
        SerializationMethod::Json,
    ) {
        Ok(loaded_db) => {
            println!("loaded DB");
            loaded_db
        }
        _ => {
            println!("creating new DB");
            PickleDb::new(
                db_path,
                PickleDbDumpPolicy::DumpUponRequest,
                SerializationMethod::Json,
            )
        }
    }
}

pub struct Database {
    pub db: Mutex<PickleDb>,
}
