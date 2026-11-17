use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct AssetsIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    #[serde(rename = "totalSize")]
    pub total_size: u64,
    pub url: String,
}

#[derive(Deserialize, Serialize)]
pub struct Index {
    pub objects: HashMap<String, Object>
}

#[derive(Deserialize, Serialize)]
pub struct Object {
    pub hash: String,
    pub size: u64,
}