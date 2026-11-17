use std::collections::HashMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::Allowed;

pub type Rules = Vec<Rule>;
pub type Features = HashMap<String, bool>;

#[derive(Deserialize, Serialize, Clone)]
pub struct Rule {
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<Os>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Features>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Os {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "versionRange", skip_serializing_if = "Option::is_none")]
    pub version_range: Option<HashMap<String, String>>,
}

impl Allowed for Rules {
    fn allowed(&self) -> bool {
        let mut allowed = false;
        for rule in self {
            if rule.matches() {
                allowed = rule.action.eq("allow");
            }
        }
        allowed
    }
}

impl Allowed for Rule {
    fn allowed(&self) -> bool {
        self.matches() && self.action.eq("allow")
    }
}

impl Rule {
    fn matches(&self) -> bool {
        if let Some(os) = &self.os {
            if !os.allowed() {
                return false;
            }
        }
        if let Some(_) = &self.features {
            return false;
        }
        true
    }
}

impl Allowed for Os {
    fn allowed(&self) -> bool {
        let mut allowed = true;
        if let Some(name) = &self.name {
            if name.eq("windows") && !cfg!(target_os = "windows") {
                allowed = false;
            } else if name.eq("linux") && !cfg!(target_os = "linux") {
                allowed = false;
            } else if name.eq("osx") && !cfg!(target_os = "macos") {
                allowed = false;
            }
        }

        if let Some(arch) = &self.arch {
            if arch.eq("x86_64") && !cfg!(target_arch = "x86_64") {
                allowed = false;
            } else if arch.eq("x86") && !cfg!(target_arch = "x86") {
                allowed = false;
            } else if arch.eq("arm64") && !cfg!(target_arch = "aarch64") {
                allowed = false;
            }
        }

        if let Some(expect_version) = &self.version {
            if !os_regex_is_match(expect_version, get_os_version().as_str()) {
                allowed = false;
            }
        }

        if let Some(version_range) = &self.version_range {
            for (k, v) in version_range {
                let expect_version = os_version_to_nums(v.as_str());
                let current_version = os_version_to_nums(get_os_version().as_str());
                if k.eq("min") {
                    if expect_version.0 > current_version.0 {
                        allowed = false;
                    } else if expect_version.1 > current_version.1 {
                        allowed = false;
                    } else if expect_version.2 > current_version.2 {
                        allowed = false;
                    }
                }else if k.eq("max") {
                    if expect_version.0 < current_version.0 {
                        allowed = false;
                    } else if expect_version.1 < current_version.1 {
                        allowed = false;
                    } else if expect_version.2 < current_version.2 {
                        allowed = false;
                    }
                }
            }
        }

        allowed
    }
}

fn os_regex_is_match(expect_version: &str, current_version: &str) -> bool {
    let Some(regex) = Regex::new(expect_version).map_err(|err| eprintln!("{}", err)).ok() else {
        return false;
    };
    regex.is_match(current_version)
}

fn os_version_to_nums(version: &str) -> (u32, u32, u32) {
    let splits = version.split('.').collect::<Vec<&str>>();
    let first = splits[0].parse::<u32>().unwrap_or(0);
    let second = splits[1].parse::<u32>().unwrap_or(0);
    let third = splits[2].parse::<u32>().unwrap_or(0);
    (first, second, third)
}

fn get_os_version() -> String {
    os_info::get().version().to_string()
}