use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("http downloader error: {0}")]
    HttpDownloaderError(#[from] http_downloader::error::Error),

    #[error("serde_json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("quick_xml error: {0}")]
    QuickXmlError(#[from] quick_xml::de::DeError),

    #[error("zip error: {0}")]
    ZipError(#[from] zip::result::ZipError),
    
    #[error("patch failed")]
    PatchFailedError,
}