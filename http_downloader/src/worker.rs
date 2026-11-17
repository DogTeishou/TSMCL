use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use futures::TryStreamExt;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use crate::error::Error;
use crate::progressor::Progressor;

pub struct Task {
    pub url: String,
    pub path: PathBuf,
    pub size: Option<u64>,
    pub sha1: Option<String>,
}


pub struct Worker {
    client: Arc<Client>,
    tasks: Arc<Mutex<Vec<Task>>>,
    progressor: Arc<Progressor>,
    canceled: AtomicBool,
}

impl Worker {
    pub fn new(client: Arc<Client>, tasks: Arc<Mutex<Vec<Task>>>, progressor: Arc<Progressor>) -> Self {
        Worker { client, tasks, progressor, canceled: AtomicBool::new(false) }
    }

    pub async fn work(self) -> Result<Self, Error> {
        loop {
            if self.tasks.lock().await.is_empty() || self.canceled.load(Ordering::Relaxed) {
                break;
            }
            let subtask = self.tasks.lock().await.pop();
            if let Some(task) = subtask {
                self.download(task).await?;
            }
        }
        Ok(self)
    }

    pub fn cancel(&mut self) {
        self.canceled.store(true, Ordering::Relaxed);
    } 

    async fn download(&self, task: Task) -> Result<(), Error> {
        let resp = self.client.get(task.url).send().await?;
        let stream = tokio_util::io::StreamReader::new(resp.bytes_stream().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)));
        let mut reader = tokio::io::BufReader::new(stream);
        if let Some(parent) = task.path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let mut file = File::create(&task.path).await?;
        let mut buf = [0u8; 1024 * 8];
        loop {
            if self.canceled.load(Ordering::Relaxed) {
                break;
            }
            let n = reader.read(&mut buf).await?;
            if n == 0 {
                break;
            }
            file.write_all(&buf[..n]).await?;
            self.progressor.add(n as u64);
        }
        file.flush().await?;
        if let Some(sha1) = task.sha1 {
            if !super::verify(&sha1, &task.path).await {
                return Err(Error::InvalidSha1);
            }
        }
        Ok(())
    }
}