use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::Allowed;
use crate::rule::Rules;

pub type Libraries = Vec<Library>;

#[derive(Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Library {
    Normal(NormalLibrary),
    Fabric(FabricLibrary),
}

#[derive(Deserialize, Serialize, Clone)]
pub struct FabricLibrary {
    pub name: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha512: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct NormalLibrary {
    pub name: String,
    pub downloads: Download,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Rules>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract: Option<Extract>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Extract {
    pub exclude: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Download {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<DownloadItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<HashMap<String, DownloadItem>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DownloadItem {
    pub path: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

impl Allowed for Library {
    fn allowed(&self) -> bool {
        match self {
            Library::Normal(normal_library) => normal_library.allowed(),
            Library::Fabric(_) => true,
        }
    }
}

impl FabricLibrary {
    pub fn get_path(&self) -> String {
        let (coordinates, extension) = self.name.split_once('@').unwrap_or((&self.name, "jar"));
        let mut parts = coordinates.split(':');
        let group = parts.next().unwrap_or_default().replace('.', "/");
        let artifact = parts.next().unwrap_or_default();
        let version = parts.next().unwrap_or_default();
        let classifier = parts.next().filter(|part| !part.is_empty());
        let classifier = classifier.map_or(String::new(), |part| format!("-{part}"));

        let separator = if cfg!(target_os = "windows") { '\\' } else { '/' };
        format!(
            "{group}{separator}{artifact}{separator}{version}{separator}{artifact}-{version}{classifier}.{extension}"
        )
    }

    pub fn get_url(&self) -> String {
        let path = self.get_path().replace('\\', "/");
        format!("{}/{}", self.url.trim_end_matches('/'), path)
    }
}

impl Allowed for NormalLibrary {
    fn allowed(&self) -> bool {
        let mut allowed = true;
        if let Some(rules) = &self.rules {
            allowed = rules.allowed();
        }

        let natives = self.get_natives();
        if let Some(native) = natives {
            if native.eq("windows") && !cfg!(target_os = "windows") {
                allowed = false;
            } else if native.eq("osx") && !cfg!(target_os = "macos") {
                allowed = false;
            } else if native.eq("linux") && !cfg!(target_os = "linux") {
                allowed = false;
            }
        }

        if self.name.contains("native") {
            if self.name.contains("x86") && !cfg!(target_arch = "x86") {
                return false;
            } else if self.name.contains("arm64") && !cfg!(target_arch = "aarch64") {
                return false;
            } else if !cfg!(target_arch = "x86_64") {
                return false;
            }
        }

        allowed
    }
}

impl NormalLibrary {
    pub fn get_natives(&self) -> Option<String> {
        let natives = self.natives.as_ref()?;

        natives.iter().find_map(|(platform, native)| {
            let matches_current_os = (platform.contains("windows") && cfg!(target_os = "windows"))
                || (platform.contains("linux") && cfg!(target_os = "linux"))
                || (platform.contains("osx") && cfg!(target_os = "macos"));

            matches_current_os.then(|| native.clone())
        })
    }
}