use http_downloader::{Downloader, terminal_sink};

use crate::{
    forge::{get_forge_installer, get_forge_metadata_official, get_forge_recommended_official}, 
    source::Source, 
    vanilla::{download_vanilla_game, get_version_manifests, get_version_url}
};

#[tokio::test]
async fn test_download_1_12_2() -> Result<(), Box<dyn std::error::Error>> {
    let game_dir = &std::env::current_dir()?.parent().unwrap().join(".minecraft");
    if !game_dir.exists() {
        std::fs::create_dir_all(game_dir)?;
    }
    let source = &Source::Official;
    let downloader = Downloader::new(64)?;
    downloader.on_progress(terminal_sink).await;

    let manifests = get_version_manifests(source).await?;
    let url = get_version_url(&manifests, "1.12.2");

    download_vanilla_game("1.12.2", &url, game_dir, source, &downloader).await?;

    Ok(())
}

#[tokio::test]
async fn test_download_26_3() -> Result<(), Box<dyn std::error::Error>> {
    let game_dir = &std::env::current_dir()?.parent().unwrap().join(".minecraft");
    if !game_dir.exists() {
        std::fs::create_dir_all(game_dir)?;
    }
    let source = &Source::Official;
    let downloader = Downloader::new(64)?;
    downloader.on_progress(terminal_sink).await;

    let manifests = get_version_manifests(source).await?;
    let url = get_version_url(&manifests, "26.3");
    
    download_vanilla_game("26.3", &url, game_dir, source, &downloader).await?;

    Ok(())
}

#[tokio::test]
async fn get_metadata() -> Result<(), Box<dyn std::error::Error>> {
    let str = get_forge_metadata_official().await?;
    println!("{:?}", str);
    Ok(())
}

#[tokio::test]
async fn get_forge_recommended() -> Result<(), Box<dyn std::error::Error>> {
    let str = get_forge_recommended_official().await?;
    println!("{:?}", str);
    Ok(())
}

#[tokio::test]
async fn get_installer() -> Result<(), Box<dyn std::error::Error>> {
    let dir = std::env::current_dir().unwrap().parent().unwrap().join(".tsmcl").join("tmp").join("forge").join("26.3-66.1.5");
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }
    get_forge_installer("26.3", "66.1.5", &dir).await?;
    Ok(())
}
