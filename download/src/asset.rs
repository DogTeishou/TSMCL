use std::path::Path;
use http_downloader::{Downloader, Task, get, verify};
use model::{AssetsIndex, Index};
use parse::Parse;
use crate::BuildTasks;
use crate::error::Error;
use crate::source::Source;

impl BuildTasks for AssetsIndex {
    async fn build_tasks(&self, downloader: &Downloader, source: &Source, game_dir: &Path) -> Result<(), Error> {
        let indexes_dir = &game_dir.join("assets").join("indexes");
        if !indexes_dir.exists() {
            tokio::fs::create_dir_all(&indexes_dir).await?;
        }

        let index_path = &indexes_dir.join(format!("{}.json", self.id));
        tokio::fs::File::create(index_path).await?;
        let url = self.url.clone();
        let text = get(url).await?;
        let index = Index::parse(&text)?;
        let text = serde_json::to_string_pretty(&index)?;
        tokio::fs::write(index_path, &text).await?;

        let objects_dir = &game_dir.join("assets").join("objects");
        if !objects_dir.exists() {
            tokio::fs::create_dir_all(&objects_dir).await?;
        }
        
        for (_, value) in index.objects {
            let hash = &value.hash;
            let hash_first_two = &hash[..2];

            let first_two_dir = &objects_dir.join(hash_first_two);
            if !first_two_dir.exists() {
                tokio::fs::create_dir_all(first_two_dir).await?;
            }

            if !first_two_dir.exists() {
                tokio::fs::create_dir_all(first_two_dir).await?;
            }

            let path = &first_two_dir.join(&hash);
            if path.exists() {
                if verify(hash, path).await {
                    continue;
                } else {
                    tokio::fs::remove_file(path).await?;
                }
            }

            let url = source.get_asset_url(&format!("https://resources.download.minecraft.net/{}/{}", hash_first_two, hash)).to_string();

            let task = Task {
                url,
                path: path.to_path_buf(),
                sha1: Some(hash.clone()),
                size: Some(value.size)
            };

            downloader.add_task(task).await;
        }

        Ok(())
    }
}