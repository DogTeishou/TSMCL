use std::path::Path;

use http_downloader::get_file;

use crate::{error::Error, extract_installer, lib_copy};

pub async fn get_forge_installer_official(mc_version: &str, forge_version: &str, tsmcl_cfg_dir: &Path, libraries_dir: &Path) -> Result<(), Error> {
    let version = format!("{}-{}", mc_version, forge_version);
    let dir = tsmcl_cfg_dir.join("tmp").join(format!("forge-{}", &version));
    if !dir.exists() {
        tokio::fs::create_dir_all(&dir).await?;
    }
    let forge_installer_path = dir.join("forge_installer.jar");
    if !forge_installer_path.exists() {
        let url = format!("https://maven.minecraftforge.net/net/minecraftforge/forge/{}/forge-{}-installer.jar", &version, &version);
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
