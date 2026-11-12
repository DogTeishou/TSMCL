use model::Index;
use crate::Parse;

impl Parse<&str> for Index {
    type Error = serde_json::Error;

    fn parse(data: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(data)
    }
}