// https://github.com/tauri-apps/tauri/blob/dev/examples/state/main.rs#L22-L23

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

pub fn get_database() -> PickleDb {
    match PickleDb::load(
        "example.db",
        PickleDbDumpPolicy::DumpUponRequest,
        SerializationMethod::Json,
    ) {
        Ok(loaded_db) => loaded_db,
        _ => PickleDb::new(
            "example.db",
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        ),
    }
}
