use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Io error: {0}")]
    IoError(std::io::Error),
    #[error("serde_json error: {0}")]
    SerdeJsonError(serde_json::Error),
    #[error("zip error: {0}")]
    ZipError(zip::result::ZipError),
}