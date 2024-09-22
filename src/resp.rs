use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Setu {
    pub error: String,
    pub data: Vec<SetuData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetuData {
    pub pid: i64,
    pub p: i64,
    pub uid: i64,
    pub title: String,
    pub author: String,
    pub r18: bool,
    pub width: i64,
    pub height: i64,
    pub tags: Vec<String>,
    pub ext: String,
    #[serde(rename = "aiType")]
    pub ai_type: i64,
    #[serde(rename = "uploadDate")]
    pub upload_date: i64,
    pub urls: SetuUrls,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetuUrls {
    pub original: Option<String>,
    pub regular: Option<String>,
    pub small: Option<String>,
    pub thumb: Option<String>,
    pub mini: Option<String>,
}
