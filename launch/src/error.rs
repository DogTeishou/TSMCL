use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("serde_json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("zip error: {0}")]
    ZipError(#[from] zip::result::ZipError),
}