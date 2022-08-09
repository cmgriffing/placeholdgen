// https://github.com/tauri-apps/tauri/blob/dev/examples/state/main.rs#L22-L23

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

pub fn get_database() -> PickleDb {
    match PickleDb::load(
        "example.db",
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    ) {
        Ok(loaded_db) => {
            println!("loaded DB");
            loaded_db
        }
        _ => {
            println!("creating new DB");
            PickleDb::new(
                "example.db",
                PickleDbDumpPolicy::AutoDump,
                SerializationMethod::Json,
            )
        }
    }
}
