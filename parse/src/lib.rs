pub mod version;
pub mod vanilla_manifest;
pub mod asset;
pub mod forge;
pub mod fabric;

pub trait Parse<T>: Sized {
    type Error;
    fn parse(data: &str) -> Result<Self, Self::Error>;
}
