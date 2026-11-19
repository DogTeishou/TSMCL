use crate::error::Error;

pub mod player;
pub mod microsoft;
pub mod error;

pub mod xbox;
pub mod minecraft;
mod crypto;

const SCOPE: &str = "XboxLive.signin offline_access";

pub(crate) fn get_client_id() -> Result<&'static str, Error> {
    option_env!("TSMCL_CLIENT_ID").ok_or(Error::MissingClientId)
}