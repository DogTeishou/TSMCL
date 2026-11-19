use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("toml ser error: {0}")]
    TomlSerError(#[from] toml::ser::Error),
    #[error("toml de error: {0}")]
    TomlDeError(#[from] toml::de::Error),
    #[error("missing client id")]
    MissingClientId,
    #[error("authorization declined")]
    AuthorizationDeclined,
    #[error("bad verification code")]
    BadVerificationCode,
    #[error("expired token")]
    ExpiredToken,
    #[error("microsoft error: {0}")]
    MicrosoftError(String),
    #[error("xbox xui mismatch")]
    XboxXuiMismatch,
    #[error("no minecraft account")]
    NoMinecraftAccount,
    #[error("no refresh token")]
    NoRefreshToken,
    #[error("keyring error: {0}")]
    KeyringError(#[from] keyring::Error),
    #[error("invalid encryption key in credential store")]
    InvalidKey,
    #[error("failed to encrypt account data")]
    Encrypt,
    #[error("failed to decrypt account data")]
    Decrypt,
}