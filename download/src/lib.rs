use std::path::Path;
use http_downloader::Downloader;

pub mod library;
pub mod source;
pub mod error;
pub mod asset;
pub mod vanilla;
pub mod forge;

pub use vanilla::download_vanilla_game;
pub use source::Source;

use crate::error::Error;

#[cfg(test)]
mod tests;

pub trait BuildTasks {
    fn build_tasks(&self, downloader: &Downloader, source: &Source, game_dir: &Path) -> impl Future<Output = Result<(), Error>>;
}