use serde::Deserialize;

#[derive(Deserialize)]
pub struct VanillaManifests {
    pub latest: Latest,
    pub versions: Vec<VanillaManifest>,
}

#[derive(Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Deserialize)]
pub struct VanillaManifest {
    pub id: String,
    #[serde(alias = "type")]
    pub type_: String,
    pub url: String,
    pub time: String,
    #[serde(alias = "releaseTime")]
    pub release_time: String,
}