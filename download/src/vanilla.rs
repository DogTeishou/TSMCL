use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use http_downloader::{Downloader, Task, get, verify};
use model::VanillaManifests;
use model::version::Version;
use parse::Parse;
use crate::BuildTasks;
use crate::error::Error;
use crate::source::Source;

pub async fn get_version_manifests(source: &Source) -> Result<VanillaManifests, Error> {
    let manifests_text = get(source.get_manifests_url("https://piston-meta.mojang.com/mc/game/version_manifest.json")).await?;
    Ok(VanillaManifests::parse(&manifests_text)?)
}

pub fn get_version_url(manifests: &VanillaManifests, game_id: &str) -> String {
    let mut url = String::new();
    manifests.versions.iter().for_each(|v| if v.id.eq(game_id) { url = v.url.clone(); });
    url
}

pub async fn download_vanilla_game(
        name: &str, 
        game_url: &str, 
        game_dir: &PathBuf, 
        source: &Source,
        downloader: &Downloader) -> Result<Version, Error> {
    let version_dir = &game_dir.join("versions").join(name);
    let version_json = &version_dir.join(format!("{}.json", name));
    if !version_dir.exists() {
        tokio::fs::create_dir_all(version_dir).await?;
    }

    
    if !version_json.exists() {    
        tokio::fs::File::create(version_json).await?;
    }

    let game_url = source.get_manifest_url(game_url);

    let version_text = get(game_url).await?;
    let vanilla = Version::parse(&version_text)?;

    let version_text = serde_json::to_string_pretty(&vanilla)?;
    tokio::fs::write(version_json, version_text).await?;

    vanilla.build_tasks(&downloader, source, version_dir).await?;
    vanilla.libraries.build_tasks(&downloader, source, game_dir).await?;
    vanilla.assets_index.build_tasks(&downloader, source, game_dir).await?;

    downloader.download().await?;

    Ok(vanilla)
}

impl BuildTasks for Version {
    async fn build_tasks(&self, downloader: &Downloader, source: &Source, version_dir: &Path) -> Result<(), Error> {
        if !version_dir.exists() {
            tokio::fs::create_dir_all(version_dir).await?;
        }

        let name = version_dir.file_name().unwrap_or(OsStr::new("")).to_string_lossy().to_string();

        let url = source.get_version_url(&self.downloads["client"].url).to_string();
        let client_path = &version_dir.join(format!("{}.jar", name));

        if client_path.exists() {
            if verify(&self.downloads["client"].sha1, client_path).await {
                return Ok(());
            }
        }

        let task = Task {
            url,
            path: client_path.clone(),
            sha1: Some(self.downloads["client"].sha1.clone()),
            size: Some(self.downloads["client"].size)
        };

        downloader.add_task(task).await;

        Ok(())
    }
}