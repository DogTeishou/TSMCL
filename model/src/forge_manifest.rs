use std::collections::HashMap;
use serde::Deserialize;

use crate::{Libraries, processor::Processor};

pub type Data = HashMap<String, Patched>;
    
#[derive(Deserialize)]
pub struct Patched {
    pub client: String,
    pub server: String,
}

#[derive(Deserialize)]
pub struct ForgeInstallProfile {
    pub spec: u32,
    pub processors: Option<Vec<Processor>>,
    #[serde(rename = "serverJarPath")]
    pub server_jar_path: String,
    pub data: Option<Data>,
    pub libraries: Libraries,
}

//Official Forge
pub type Promos = HashMap<String, String>;

#[derive(Deserialize, Debug)]
pub struct ForgeRecommended {
    pub homepage: String,
    pub promos: Promos,
}

#[derive(Deserialize, Debug)]
pub struct ForgeMetadata {
    pub versioning: Versioning,
}

#[derive(Deserialize, Debug)]
pub struct Versioning {
    pub latest: Option<String>,
    pub release: Option<String>,
    pub versions: Vec<ForgeVersion>,
}

#[derive(Deserialize, Debug)]
pub struct ForgeVersion {
    #[serde(default)]
    pub version: Vec<String>,
}

impl ForgeVersion {
    pub fn get_forge_version(&self, mc_id: &str) -> Vec<String> {
        let mut versions = vec![];
        for v in &self.version {
            let splits: Vec<&str> = v.split("-").collect();
            let minecraft_version = splits[0];
            let forge_version = splits[1];
            if minecraft_version.eq(mc_id) {
                versions.push(forge_version.to_string());
            }
        }
        versions
    }
}