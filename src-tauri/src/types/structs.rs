use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Clone)]
#[ts(export)]
pub struct Site {
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, TS, Clone)]
#[ts(export)]
pub struct ExternalSiteImage {
    pub id: String,
    pub uploaded: bool,
    pub source_url: String,
}

#[derive(Serialize, Deserialize, TS, Clone)]
#[ts(export)]
pub struct LocalSiteImage {
    pub id: String,
    pub uploaded: bool,
    pub original_file_name: String,
}

#[derive(Serialize, Deserialize, TS, Clone)]
#[ts(export)]
pub struct Collection {
    pub collection_id: String,
    pub name: String,
    // images: Vec<SiteImage>,
}

#[derive(Serialize, Deserialize, TS, Clone)]
#[ts(export)]
pub struct Settings {
    pub vercel_api_key: String,
}
