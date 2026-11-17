use http_downloader::get;
use model::fabric::FabricVersion;
use parse::Parse;

use crate::{Source, error::Error};

pub async fn get_fabric_version(source: &Source, mc_ver: &str, fab_ver: &str) -> Result<FabricVersion, Error> {
    let url = format!("https://meta.fabricmc.net/v2/versions/loader/{mc_ver}/{fab_ver}/profile/json");
    let url = source.get_fabric_url(&url);
    let text = get(url).await?;
    Ok(FabricVersion::parse(&text)?)
}