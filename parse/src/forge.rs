use model::{forge::ForgeVersion, forge_manifest::{ForgeInstallProfile, ForgeMetadata, ForgeRecommended}};


use crate::Parse;

impl Parse<&str> for ForgeMetadata {
    type Error = quick_xml::DeError;

    fn parse(data: &str) -> Result<Self, Self::Error> {
        quick_xml::de::from_str(data)
    }
}

impl Parse<&str> for ForgeRecommended {
    type Error = serde_json::Error;

    fn parse(data: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(data)
    }
}

impl Parse<&str> for ForgeVersion {
    type Error = serde_json::Error;

    fn parse(data: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(data)
    }
}

impl Parse<&str> for ForgeInstallProfile {
    type Error = serde_json::Error;

    fn parse(data: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(data)
    }
}