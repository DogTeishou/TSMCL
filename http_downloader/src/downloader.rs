use std::sync::Arc;
use reqwest::Client;
use tokio::sync::Mutex;
use crate::progressor::Snapshot;
use crate::error::Error;
use crate::progressor::{Progressor, Status};
use crate::worker::{Task, Worker};

const MAX_WORKERS: usize = 256;

pub struct Downloader {
    max_workers: usize,
    tasks: Arc<Mutex<Vec<Task>>>,
    canceled: bool,
    client: Arc<Client>,
    progressor: Arc<Progressor>,
}

impl Downloader {
    pub fn new(max_workers: usize) -> Result<Self, Error> {
        let max_workers = max_workers.min(MAX_WORKERS);
        let client = Arc::new(reqwest::ClientBuilder::new().pool_max_idle_per_host(max_workers).build()?);
        let progressor = Progressor::new();
        Ok(Downloader {
            canceled: false,
            max_workers,
            tasks: Arc::new(Mutex::new(Vec::new())),
            client,
            progressor,
        })
    }

    pub async fn on_progress(&self, sink: impl Fn(Snapshot) + Send + Sync + 'static) {
        self.progressor.set_sink(Arc::new(sink)).await;
    }

    pub async fn add_task(&self, task: Task) {
        if let Some(size) = task.size {
            self.progressor.add_total(size);
        }
        self.tasks.lock().await.push(task);
    }

    pub fn cancel(&mut self) {
        self.canceled = true;
    }

    pub async fn download(&self) -> Result<(), Error> {
        self.progressor.set_status(Status::Downloading).await;
        let reporter = self.progressor.spawn_reporter();
        let mut handles = Vec::new();
        for _ in 0..self.max_workers {
            let tasks = self.tasks.clone();
            let client = self.client.clone();
            let progressor = self.progressor.clone();
            let handle = tokio::spawn(async move {
                let worker = Worker::new(client, tasks, progressor);
                worker.work().await
            });
            handles.push(handle);
        }

        let mut first_err: Option<Error> = None;
        for h in handles {
            match h.await {
                Ok(Ok(mut worker)) => {
                    if self.canceled {
                        worker.cancel();
                    }
                }
                Ok(Err(e)) => {  
                    first_err = Some(e);
                }
                Err(_) => {
                    first_err = Some(Error::WorkerPanic);
                }
            }
        }
        if let Some(e) = first_err {
            self.progressor.fail(e.to_string()).await;
            self.progressor.finish();
            let _ = reporter.await;
            return Err(e);
        }
        self.progressor.set_status(Status::Done).await;
        self.progressor.finish();
        let _ = reporter.await;
        Ok(())
    }
}