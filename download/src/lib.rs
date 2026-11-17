use std::path::Path;
use http_downloader::Downloader;

pub mod library;
pub mod source;
pub mod error;
pub mod asset;
pub mod vanilla;
pub mod forge;
pub mod patch;
pub mod neoforge;
pub mod fabric;
pub mod quilt;

pub use vanilla::download_vanilla_game;
pub use source::Source;
use zip::ZipArchive;

use crate::error::Error;

#[cfg(test)]
mod tests;

pub trait BuildTasks {
    fn build_tasks(&self, downloader: &Downloader, source: &Source, game_dir: &Path) -> impl Future<Output = Result<(), Error>>;
}

pub trait Patch {
    type Version;
    fn patch(&self, downloader: &Downloader, source: &Source, jar_dir: &str, tmp_dir: &Path, ver_dir: &Path) -> impl Future<Output = Result<Self::Version, Error>>;
}

pub fn lib_copy(dir: &Path, copied_dir: &Path) -> Result<(), Error> {
    let reader = dir.read_dir()?;
    let dir_str = dir.to_string_lossy().to_string();
    for m_dir in reader {
        let path = m_dir?.path();
        if path.is_dir() {
            let path_str = path.to_string_lossy().to_string().replace(&dir_str, "");
            let path_str = if cfg!(target_os = "windows") {
                path_str.replace("\\", "")
            } else {
                path_str.replace("/", "")
            };
            let copied = copied_dir.join(path_str);
            lib_copy(&path, &copied)?;
        } else {
            if !copied_dir.exists() {
                std::fs::create_dir_all(copied_dir)?;
            }
            let file_str = path.to_string_lossy().to_string().replace(&dir_str, "");
            let file_str = if cfg!(target_os = "windows") {
                file_str.replace("\\", "")
            } else {
                file_str.replace("/", "")
            };
            let copied_file = copied_dir.join(file_str);
            std::fs::copy(path, copied_file)?;
        }
    }
    Ok(())
}

pub fn extract_installer(installer: &Path, dir: &Path) -> Result<(), Error> {
    let mut zip = ZipArchive::new(std::fs::File::open(installer)?)?;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        if entry.name()?.contains("META-INF/") {
            continue;
        }
        let mut is_not_continue = false;
        if entry.name()?.contains("maven/") {
            is_not_continue = true;
        } else if entry.name()?.contains("data/") {
            is_not_continue = true;
        } else if entry.name()?.ends_with(".json") {
            is_not_continue = true;
        }
        if !is_not_continue {
            continue;
        }
        let Some(rel) = entry.enclosed_name() else {
            continue;
        };
        let out = dir.join(rel);
        if entry.is_dir() {
            std::fs::create_dir_all(out)?;
            continue;
        } 
        if let Some(parent) = out.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        } 
        let mut file = std::fs::File::create(out)?;
        std::io::copy(&mut entry, &mut file)?;

        #[cfg(unix)]
        if let Some(mode) = entry.unix_mode() {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&out, fs::Permissions::from_mode(mode))?;
        }
    }
    Ok(())
}