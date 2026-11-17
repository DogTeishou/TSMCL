use std::path::Path;

use serde::Deserialize;

use crate::{Allowed, Library};

#[derive(Deserialize)]
pub struct Processor {
    pub sides: Option<Vec<String>>,
    pub jar: String,
    pub classpath: Vec<String>,
    pub args: Vec<String>,
}

impl Allowed for Processor {
    fn allowed(&self) -> bool {
        let mut allowed = false;
        if let Some(sides) = &self.sides {
            for v in sides {
                if v.contains("client") {
                    allowed = true;
                }
            }
        } else {
            allowed = true
        }
        allowed
    }
}

impl Processor {
    pub fn get_classpath(&self, libraries: &[Library], libraries_dir: &Path) -> String {
        let mut vec = Vec::new();
        for class in &self.classpath {
            for lib in libraries {
                let lib = match lib {
                    Library::Normal(lib) => lib,
                    Library::Fabric(_) => continue,
                };
                if let Some(artifact) = &lib.downloads.artifact {
                    if lib.name.eq(class) {
                        vec.push(libraries_dir.join(artifact.path.clone()).to_string_lossy().into_owned());
                    }
                }
            }
        }
        if cfg!(target_os = "windows") { vec.join(";") } else { vec.join(":") }
    }
}