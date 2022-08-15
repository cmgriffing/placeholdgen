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
    pub collection_name: String,
    pub collection_key: String,
    pub site_id: String,
}

#[derive(Serialize, Deserialize, TS, Clone, Default, Debug)]
#[ts(export)]
pub struct Site {
    pub site_name: String,
    pub site_id: String,
}

#[derive(Serialize, Deserialize, TS, Clone, Default, Debug)]
#[ts(export)]
pub struct SiteImage {
    pub image_id: String,
    pub uploaded: bool,
    pub original_file_name: String,
    pub source_url: String,
    pub image_type: String,
    pub collection_id: String,
}

#[derive(Serialize, Deserialize, TS, Clone, Default, Debug)]
#[ts(export)]
pub struct Settings {
    pub vercel_api_key: String,
}
