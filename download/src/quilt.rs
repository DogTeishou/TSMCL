use http_downloader::get;
use model::fabric::FabricVersion;
use parse::Parse;

use crate::{Source, error::Error};

pub async fn get_quilt_version(source: &Source, mc_ver: &str, qui_ver: &str) -> Result<FabricVersion, Error> {
    let url = format!("https://meta.quiltmc.org/v3/versions/loader/{mc_ver}/{qui_ver}/profile/json");
    let url = source.get_quilt_url(&url);
    let text = get(url).await?;
    Ok(FabricVersion::parse(&text)?)
}