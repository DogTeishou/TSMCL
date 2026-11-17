use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Not implemented")]
    NotImplemented,
    #[error("Invalid SHA-1 hash")]
    InvalidSha1,
    #[error("Worker panic")]
    WorkerPanic,
}