use crate::Parse;
use model::version::Version;

impl Parse<&str> for Version {
    type Error = serde_json::error::Error;

    fn parse(data: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Self>(data)
    }
}