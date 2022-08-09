use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Clone, Default, Debug)]
#[ts(export)]
pub struct AppState {
    pub sites: Vec<Site>,
}

#[derive(Serialize, Deserialize, TS, Clone, Default, Debug)]
#[ts(export)]
pub struct Collection {
    pub collection_id: String,
    pub name: String,
    images: Vec<SiteImage>,
}

#[derive(Serialize, Deserialize, TS, Clone, Default, Debug)]
#[ts(export)]
pub struct Site {
    pub name: String,
    pub id: String,
    pub collections: Vec<Collection>,
}

#[derive(Serialize, Deserialize, TS, Clone, Default, Debug)]
#[ts(export)]
pub struct SiteImage {
    pub id: String,
    pub uploaded: bool,
    pub original_file_name: String,
    pub source_url: String,
}

#[derive(Serialize, Deserialize, TS, Clone, Default, Debug)]
#[ts(export)]
pub struct Settings {
    pub vercel_api_key: String,
}
