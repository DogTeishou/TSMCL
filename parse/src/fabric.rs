use model::fabric::FabricVersion;

use crate::Parse;

impl Parse<&str> for FabricVersion {
    type Error = serde_json::Error;

    fn parse(data: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(data)
    }
}