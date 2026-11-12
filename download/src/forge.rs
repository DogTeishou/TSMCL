use std::{f64::consts::E, path::{Path, PathBuf}};

use http_downloader::{get, get_file};
use model::forge_manifest::{ 
    ForgeMetadata, 
    ForgeRecommended};
use parse::Parse;
use zip::ZipArchive;

use crate::{BuildTasks, error::Error};

pub async fn get_forge_metadata_official() -> Result<ForgeMetadata, Error> {
    let text = get("https://maven.minecraftforge.net/net/minecraftforge/forge/maven-metadata.xml")
        .await
        .map_err(|e| Error::HttpDownloaderError(e))?;
    ForgeMetadata::parse(&text).map_err(|e| Error::QuickXmlError(e))
}

pub async fn get_forge_recommended_official() -> Result<ForgeRecommended, Error> {
    let text = get("https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json")
        .await
        .map_err(|e| Error::HttpDownloaderError(e))?;
    ForgeRecommended::parse(&text).map_err(|e| Error::SerdeJsonError(e))
}

pub async fn get_forge_installer(mc_version: &str, forge_version: &str, dir: &PathBuf) -> Result<(), Error> {
    let version = format!("{}-{}", mc_version, forge_version);
    let file = dir.join(format!("forge-{}-installer.jar", version));
    get_file(format!("https://maven.minecraftforge.net/net/minecraftforge/forge/{}/forge-{}-installer.jar", version, version), &file)
        .await
        .map_err(|e| Error::HttpDownloaderError(e))?;
    extract_install_jar(&file, dir).map_err(|e| Error::IoError(e))?;
    Ok(())
}

pub fn extract_install_jar(jar: &Path, path: &Path) -> Result<(), std::io::Error> {
    let mut archive = ZipArchive::new(std::fs::File::open(jar)?)?;
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let mut b = false;
        if entry.name()?.contains(".json") {
            b = true;
        } else if entry.name()?.contains("maven/") && !entry.name()?.contains("META-INF/") {
            b = true;
        } else if entry.name()?.contains("data/") {
            b = true;
        }
        if !b {
            continue;
        }
        let Some(rel) = entry.enclosed_name() else {
            continue;
        };
        let out = path.join(rel);

        if entry.is_dir() {
            std::fs::create_dir_all(&out)?;
            continue;
        }

        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut file = std::fs::File::create(&out)?;
        std::io::copy(&mut entry, &mut file)?;

        // Unix 上保留可执行权限等
        #[cfg(unix)]
        if let Some(mode) = entry.unix_mode() {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&out, std::fs::Permissions::from_mode(mode))?;
        }
    }
    Ok(())
}


pub async fn download_forge(
    downloader: &http_downloader::Downloader,
    mc_version: &str,
    forge_version: &str,
    source: &crate::Source, 
    version_dir: &Path, 
    tmp_dir: &Path) -> Result<(), Error> 
{

    Ok(())
}
