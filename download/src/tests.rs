use http_downloader::{Downloader, terminal_sink};
use model::{VersionTrait, forge_manifest::ForgeInstallProfile};
use parse::Parse;

use crate::{
    BuildTasks, Patch, fabric::get_fabric_version, forge::get_forge_installer_official, neoforge::get_neoforge_installer_official, quilt::get_quilt_version, source::Source, vanilla::{download_vanilla_game, get_version_manifests, get_version_url}
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
async fn get_forge_installer() -> Result<(), Box<dyn std::error::Error>> {
    let tsmcl_dir = &std::env::current_dir()?.parent().unwrap().join(".tsmcl");
    let libraries = &std::env::current_dir()?.parent().unwrap().join(".minecraft").join("libraries");
    if !tsmcl_dir.exists() {
        tokio::fs::create_dir_all(tsmcl_dir).await?;
    }
    if !libraries.exists() {
        tokio::fs::create_dir_all(libraries).await?;
    }
    get_forge_installer_official("26.3", "66.1.8", tsmcl_dir, libraries).await?;
    Ok(())
}

#[tokio::test]
async fn get_neoforge_installer() -> Result<(), Box<dyn std::error::Error>> {
    let tsmcl_dir = &std::env::current_dir()?.parent().unwrap().join(".tsmcl");
    let libraries = &std::env::current_dir()?.parent().unwrap().join(".minecraft").join("libraries");
    if !tsmcl_dir.exists() {
        tokio::fs::create_dir_all(tsmcl_dir).await?;
    }
    if !libraries.exists() {
        tokio::fs::create_dir_all(libraries).await?;
    }
    get_neoforge_installer_official("26.3.0.110", tsmcl_dir, libraries).await?;
    Ok(())
}

#[tokio::test]
async fn test_download_1_12_2_forge_14_23_5_2859() -> Result<(), Box<dyn std::error::Error>> {
    let tsmcl_dir = &std::env::current_dir()?.parent().unwrap().join(".tsmcl");
    let game_dir = &std::env::current_dir()?.parent().unwrap().join(".minecraft");
    let ver_dir = &game_dir.join("versions").join("forge-1.12.2-14.23.5.2859");
    let libraries = &game_dir.join("libraries");
    if !tsmcl_dir.exists() {
        tokio::fs::create_dir_all(tsmcl_dir).await?;
    }
    if !libraries.exists() {
        tokio::fs::create_dir_all(libraries).await?;
    }
    get_forge_installer_official("1.12.2", "14.23.5.2859", tsmcl_dir, libraries).await?;
    let source = &Source::Official;
    let downloader = Downloader::new(64)?;
    downloader.on_progress(terminal_sink).await;
    let manifests = get_version_manifests(source).await?;
    let url = get_version_url(&manifests, "1.12.2");
    let mut vanilla= download_vanilla_game("forge-1.12.2-14.23.5.2859", &url, game_dir, source, &downloader).await?;
    let tmp_dir = &tsmcl_dir.join("tmp").join("forge-1.12.2-14.23.5.2859");
    let profile_path = &tmp_dir.join("install_profile.json");
    let profile = ForgeInstallProfile::parse(&std::fs::read_to_string(profile_path)?)?;
    let forge_version = profile.patch(&downloader, source, "", tmp_dir, ver_dir).await?;
    forge_version.libraries.build_tasks(&downloader, source, game_dir).await?;
    downloader.download().await?;
    let json_path = &ver_dir.join("forge-1.12.2-14.23.5.2859.json");
    forge_version.join(&mut vanilla);
    let str = serde_json::to_string_pretty(&vanilla)?;
    std::fs::write(json_path, str)?;
    std::fs::remove_dir_all(tmp_dir)?;
    Ok(())
}

#[tokio::test]
async fn test_download_26_3_forge_66_1_8() -> Result<(), Box<dyn std::error::Error>> {
    let tsmcl_dir = &std::env::current_dir()?.parent().unwrap().join(".tsmcl");
    let game_dir = &std::env::current_dir()?.parent().unwrap().join(".minecraft");
    let ver_dir = &game_dir.join("versions").join("forge-26.3-66.1.8");
    let libraries = &game_dir.join("libraries");
    if !tsmcl_dir.exists() {
        tokio::fs::create_dir_all(tsmcl_dir).await?;
    }
    if !libraries.exists() {
        tokio::fs::create_dir_all(libraries).await?;
    }
    get_forge_installer_official("26.3", "66.1.8", tsmcl_dir, libraries).await?;
    let source = &Source::Official;
    let downloader = Downloader::new(64)?;
    downloader.on_progress(terminal_sink).await;
    let manifests = get_version_manifests(source).await?;
    let url = get_version_url(&manifests, "26.3");
    let mut vanilla= download_vanilla_game("forge-26.3-66.1.8", &url, game_dir, source, &downloader).await?;
    let tmp_dir = &tsmcl_dir.join("tmp").join("forge-26.3-66.1.8");
    let profile_path = &tmp_dir.join("install_profile.json");
    let profile = ForgeInstallProfile::parse(&std::fs::read_to_string(profile_path)?)?;
    let forge_version = profile.patch(&downloader, source, "C:\\Program Files\\Zulu\\zulu-25\\bin\\java.exe", tmp_dir, ver_dir).await?;
    forge_version.libraries.build_tasks(&downloader, source, game_dir).await?;
    downloader.download().await?;
    let json_path = &ver_dir.join("forge-26.3-66.1.8.json");
    forge_version.join(&mut vanilla);
    let str = serde_json::to_string_pretty(&vanilla)?;
    std::fs::write(json_path, str)?;
    std::fs::remove_dir_all(tmp_dir)?;
    Ok(())
}

#[tokio::test]
async fn test_download_26_3_0_110_neoforge() -> Result<(), Box<dyn std::error::Error>> {
    let tsmcl_dir = &std::env::current_dir()?.parent().unwrap().join(".tsmcl");
    let game_dir = &std::env::current_dir()?.parent().unwrap().join(".minecraft");
    let ver_dir = &game_dir.join("versions").join("neoforge-26.3-26.3.0.110");
    let libraries = &game_dir.join("libraries");
    if !tsmcl_dir.exists() {
        tokio::fs::create_dir_all(tsmcl_dir).await?;
    }
    if !libraries.exists() {
        tokio::fs::create_dir_all(libraries).await?;
    }
    get_neoforge_installer_official("26.3.0.110", tsmcl_dir, libraries).await?;
    let source = &Source::Official;
    let downloader = Downloader::new(64)?;
    downloader.on_progress(terminal_sink).await;
    let manifests = get_version_manifests(source).await?;
    let url = get_version_url(&manifests, "26.3");
    let mut vanilla= download_vanilla_game("neoforge-26.3-26.3.0.110", &url, game_dir, source, &downloader).await?;
    let tmp_dir = &tsmcl_dir.join("tmp").join("neoforge-26.3.0.110");
    let profile_path = &tmp_dir.join("install_profile.json");
    let profile = ForgeInstallProfile::parse(&std::fs::read_to_string(profile_path)?)?;
    let forge_version = profile.patch(&downloader, source, "C:\\Program Files\\Zulu\\zulu-25\\bin\\java.exe", tmp_dir, ver_dir).await?;
    forge_version.libraries.build_tasks(&downloader, source, game_dir).await?;
    downloader.download().await?;
    let json_path = &ver_dir.join("neoforge-26.3-26.3.0.110.json");
    forge_version.join(&mut vanilla);
    let str = serde_json::to_string_pretty(&vanilla)?;
    std::fs::write(json_path, str)?;
    std::fs::remove_dir_all(tmp_dir)?;
    Ok(())
}

#[tokio::test]
async fn test_download_26_3_0_20_1_fabric() -> Result<(), Box<dyn std::error::Error>> {
    let game_dir = &std::env::current_dir()?.parent().unwrap().join(".minecraft");
    let ver_dir = &game_dir.join("versions").join("fabric-26.3-0.20.1");
    let source = &Source::Official;
    let downloader = Downloader::new(64)?;
    downloader.on_progress(terminal_sink).await;
    let manifests = get_version_manifests(source).await?;
    let url = get_version_url(&manifests, "26.3");
    let mut vanilla= download_vanilla_game("fabric-26.3-0.20.1", &url, game_dir, source, &downloader).await?;
    let fabric_version = get_fabric_version(source, "26.3", "0.20.1").await?;
    fabric_version.libraries.build_tasks(&downloader, source, game_dir).await?;
    downloader.download().await?;
    let json_path = &ver_dir.join("fabric-26.3-0.20.1.json");
    fabric_version.join(&mut vanilla);
    let str = serde_json::to_string_pretty(&vanilla)?;
    std::fs::write(json_path, str)?;
    Ok(())
}

#[tokio::test]
async fn test_download_26_3_0_20_1_quilt() -> Result<(), Box<dyn std::error::Error>> {
    let game_dir = &std::env::current_dir()?.parent().unwrap().join(".minecraft");
    let ver_dir = &game_dir.join("versions").join("quilt-26.3-0.30.2");
    let source = &Source::Official;
    let downloader = Downloader::new(64)?;
    downloader.on_progress(terminal_sink).await;
    let manifests = get_version_manifests(source).await?;
    let url = get_version_url(&manifests, "26.3");
    let mut vanilla= download_vanilla_game("quilt-26.3-0.30.2", &url, game_dir, source, &downloader).await?;
    let fabric_version = get_quilt_version(source, "26.3", "0.30.2").await?;
    fabric_version.libraries.build_tasks(&downloader, source, game_dir).await?;
    downloader.download().await?;
    let json_path = &ver_dir.join("quilt-26.3-0.30.2.json");
    fabric_version.join(&mut vanilla);
    let str = serde_json::to_string_pretty(&vanilla)?;
    std::fs::write(json_path, str)?;
    Ok(())
}