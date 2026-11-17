use std::path::Path;

use crate::{error::Error, extract_installer, lib_copy};
use http_downloader::get_file;

pub async fn get_neoforge_installer_official(neoforge_version: &str, tsmcl_cfg_dir: &Path, libraries_dir: &Path) -> Result<(), Error> {
    let dir = tsmcl_cfg_dir.join("tmp").join(format!("neoforge-{}", &neoforge_version));
    if !dir.exists() {
        tokio::fs::create_dir_all(&dir).await?;
    }
    let forge_installer_path = dir.join("forge_installer.jar");
    if !forge_installer_path.exists() {
        let url = format!("https://maven.neoforged.net/releases/net/neoforged/neoforge/{}/neoforge-{}-installer.jar", &neoforge_version, &neoforge_version);
        get_file(url, &forge_installer_path).await?;
    }
    extract_installer(&forge_installer_path, &dir)?;
    let maven = dir.join("maven");
    if maven.exists() {
        lib_copy(&maven, libraries_dir)?;
    }
    tokio::fs::remove_file(forge_installer_path).await?;
    Ok(())
}