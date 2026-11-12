use model::vanilla_manifest::VanillaManifests;
use crate::Parse;

impl Parse<&str> for VanillaManifests {
    type Error = serde_json::Error;

    fn parse(data: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Self>(data)
    }
}