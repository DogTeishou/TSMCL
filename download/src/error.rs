use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("http downloader error: {0}")]
    HttpDownloaderError(http_downloader::error::Error),

    #[error("serde_json error: {0}")]
    SerdeJsonError(serde_json::Error),

    #[error("Io error: {0}")]
    IoError(std::io::Error),

    #[error("quick_xml error: {0}")]
    QuickXmlError(quick_xml::de::DeError),
}