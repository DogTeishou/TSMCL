use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::argument::Arguments;
use crate::asset::AssetsIndex;
use crate::library::Libraries;

#[derive(Deserialize, Serialize, Clone)]
pub struct Version {
    #[serde(rename = "minecraftArguments", skip_serializing_if = "Option::is_none")]
    pub minecraft_arguments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Arguments>,
    #[serde(rename = "assetIndex")]
    pub assets_index: AssetsIndex,
    pub assets: String,
    #[serde(rename = "complianceLevel")]
    pub compliance_level: u32,
    pub downloads: HashMap<String, MainJar>,
    pub id: String,
    pub libraries: Libraries,
    pub logging: HashMap<String, LoggingClient>,
    #[serde(rename = "mainClass")]
    pub main_class: String,
    #[serde(rename = "minimumLauncherVersion")]
    pub minimum_launcher_version: u32,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    pub time: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "clientVersion", skip_serializing_if = "Option::is_none")]
    pub client_version: Option<String>,
    #[serde(rename = "inheritsFrom", skip_serializing_if = "Option::is_none")]
    pub inherits_from: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct JavaVersion {
    pub component: String,
    #[serde(rename = "majorVersion")]
    pub major_version: u32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MainJar {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LoggingClient {
    pub argument: String,
    pub file: LoggingFile,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LoggingFile {
    pub id: String,
    pub sha1: String,
    pub size: usize,
    pub url: String,
}