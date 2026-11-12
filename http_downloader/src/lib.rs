use std::path::PathBuf;
use futures::TryStreamExt;
use reqwest::IntoUrl;
use sha1::Digest;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub use downloader::*;
pub use progressor::*;
pub use worker::*;

pub mod progressor;
pub mod downloader;
pub mod worker;
pub mod error;

pub async fn verify(sha1: &str, path: &PathBuf) -> bool {
    let hash = match sha1_of(path).await {
        Ok(hash) => hash,
        Err(_) => return false,
    };
    hash.eq(sha1)
}

pub async fn sha1_of(path: &PathBuf) -> Result<String, std::io::Error> {
    let mut file = File::open(path).await?;
    let mut hasher = sha1::Sha1::new();
    let mut buf = [0u8; 1024 * 8];
    loop {
        let n = file.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hex::encode(hasher.finalize()))
}

pub async fn get<U: IntoUrl>(url: U) -> Result<String, crate::error::Error> {
    let resp = reqwest::ClientBuilder::new().build()?.get(url).send().await?;
    Ok(resp.text().await?)
}

pub async fn get_file<U: IntoUrl>(url: U, path: &PathBuf) -> Result<(), crate::error::Error> {
    let resp = reqwest::ClientBuilder::new().build()?.get(url).send().await?;
    let stream = tokio_util::io::StreamReader::new(resp.bytes_stream().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)));
    let mut reader = tokio::io::BufReader::new(stream);
    let mut file;
    if path.exists() {
        file = tokio::fs::File::open(path).await?;
    } else {
        file = tokio::fs::File::create(path).await?;
    }
    let mut buf = [0u8; 8192];
    loop {
        let n = reader.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        file.write_all(&mut buf[..n]).await?;
    }
    file.flush().await?;
    Ok(())
}

pub async fn get_size<U: IntoUrl>(url: U) -> Result<u64, crate::error::Error> {
    let resp = reqwest::ClientBuilder::new().build()?.get(url).send().await?;
    if let Some(len) = resp.content_length() {
        Ok(len)
    } else {
        Ok(0)
    }
}