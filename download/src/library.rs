use std::path::Path;
use http_downloader::{Downloader, Task, verify};
use model::{Allowed, Libraries};
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
            let lib = match lib {
                model::Library::Normal(normal_library) => normal_library,
                model::Library::Fabric(fabric_library) => {
                    let path = libraries_dir.join(fabric_library.get_path());
                    if path.exists() {
                        if let Some(sha1) = &fabric_library.sha1 {
                            if verify(sha1, &path).await {
                                continue;
                            }
                            tokio::fs::remove_file(&path).await.map_err(|e| Error::IoError(e))?;
                        } else {
                            continue;
                        }
                    }
                    let task = Task {
                        url: source.get_library_url(&fabric_library.get_url()).to_string(),
                        path,
                        size: if let Some(size) = fabric_library.size { Some(size) } else { Some(0) },
                        sha1: fabric_library.sha1.clone(),
                    };
                    downloader.add_task(task).await;
                    continue;
                },
            };
            if let Some(artifact) = &lib.downloads.artifact {
                let lib_path = &libraries_dir.join(&artifact.path);
                if lib_path.exists() {
                    if !verify(&artifact.sha1, lib_path).await {
                        tokio::fs::remove_file(lib_path).await.map_err(|e| Error::IoError(e))?;
                    } else {
                        continue;
                    }
                }
                if !lib_path.parent().unwrap().exists() {
                    tokio::fs::create_dir_all(&lib_path.parent().unwrap()).await.map_err(|e| Error::IoError(e))?;
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

            if let Some(native) = lib.get_natives() {
                if let Some(download) = lib
                    .downloads
                    .classifiers
                    .as_ref()
                    .and_then(|classifiers| classifiers.get(&native))
                {
                    let url = source.get_library_url(&download.url).to_string();
                    let lib_path = &libraries_dir.join(&download.path);
                    if lib_path.exists() {
                        if !verify(&download.sha1, lib_path).await {
                            tokio::fs::remove_file(lib_path).await.map_err(|e| Error::IoError(e))?;
                        } else {
                            continue;
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
        }

        Ok(())
    }
}
