use std::path::Path;
use http_downloader::{Downloader, Task, verify};
use model::{Allowed, FabricLibrary, Libraries, NormalLibrary};
use crate::BuildTasks;
use crate::error::Error;
use crate::source::Source;

impl BuildTasks for Libraries {
    async fn build_tasks(&self, downloader: &Downloader, source: &Source, game_dir: &Path) -> Result<(), Error> {
        let libraries_dir = game_dir.join("libraries");
        for lib in self {
            if !lib.allowed() {
                continue;
            }
            match lib {
                model::Library::Normal(normal) => build_normal_task(downloader, normal, source, &libraries_dir).await?,
                model::Library::Fabric(fabric) => build_fabric_task(downloader, fabric, source, &libraries_dir).await?,
            }
        }

        Ok(())
    }
}

async fn build_fabric_task(downloader: &Downloader, fabric: &FabricLibrary, source: &Source, libraries_dir: &Path) -> Result<(), Error> {
    let path = libraries_dir.join(fabric.get_path());
        if path.exists() {
            if let Some(sha1) = &fabric.sha1 {
                if verify(sha1, &path).await {
                    return Ok(());
                }
                tokio::fs::remove_file(&path).await?;
            } else {
                return Ok(());
            }
        }
    let task = Task {
        url: source.get_library_url(&fabric.get_url()).to_string(),
        path,
        size: if let Some(size) = fabric.size { Some(size) } else { Some(0) },
        sha1: fabric.sha1.clone(),
    };
    downloader.add_task(task).await;
    Ok(())
}

async fn build_normal_task(downloader: &Downloader, normal: &NormalLibrary, source: &Source, libraries_dir: &Path) -> Result<(), Error> {
    if let Some(artifact) = &normal.downloads.artifact {
        let lib_path = &libraries_dir.join(&artifact.path);
        if lib_path.exists() {
            if !verify(&artifact.sha1, lib_path).await {
                tokio::fs::remove_file(lib_path).await?;
            } else {
                return Ok(());
            }
        }
        if !lib_path.parent().unwrap_or(Path::new("")).exists() {
            tokio::fs::create_dir_all(&lib_path.parent().unwrap_or(Path::new(""))).await?;
        }
        let url = source.get_library_url(&artifact.url).to_string();
        let task = Task {
            url,
            path: lib_path.to_path_buf(),
            sha1: Some(artifact.sha1.clone()),
            size: Some(artifact.size),
        };
        downloader.add_task(task).await; 
    }

    if let Some(native) = normal.get_natives() {
        if let Some(download) = normal
            .downloads
            .classifiers
            .as_ref()
            .and_then(|classifiers| classifiers.get(&native))
        {
            let url = source.get_library_url(&download.url).to_string();
            let lib_path = &libraries_dir.join(&download.path);
            if lib_path.exists() {
                if !verify(&download.sha1, lib_path).await {
                    tokio::fs::remove_file(lib_path).await?;
                } else {
                    return Ok(());
                }
            }
            let task = Task {
                url,
                path: lib_path.clone(),
                sha1: Some(download.sha1.clone()),
                size: Some(download.size),
            };
            downloader.add_task(task).await;
        }
    }
    Ok(())
}