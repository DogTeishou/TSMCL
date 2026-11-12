use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::argument::Arguments;
use crate::asset::AssetsIndex;
use crate::library::Libraries;

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
pub struct MainJar {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoggingClient {
    pub argument: String,
    pub file: LoggingFile,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoggingFile {
    pub id: String,
    pub sha1: String,
    pub size: usize,
    pub url: String,
}

impl Version {
    pub fn join(&mut self, version: Version) {
        if let Some(m_mc_args) = &self.minecraft_arguments {
            if let Some(mc_args) = &version.minecraft_arguments {
                self.minecraft_arguments = Some(format!("{} {}", m_mc_args, mc_args));
            }
        }

        let arguments = version.arguments;
        if let Some(m_args) = &mut self.arguments {
            if let Some(args) = arguments  {
                for arg in args.jvm {
                    m_args.jvm.push(arg);
                }
                for arg in args.game {
                    m_args.game.push(arg);
                }
                if let Some(default) = args.default_user_jvm {
                    if let Some(m_default) = &mut m_args.default_user_jvm {
                        m_default.extend(default);
                    }
                }
            }
        }

        let libraries = version.libraries;
        for lib in libraries.into_iter() {
            self.libraries.push(lib);
        }

        self.main_class = version.main_class;
    }
}