pub mod version;
pub mod argument;
pub mod rule;
pub mod library;
pub mod asset;
pub mod vanilla_manifest;
pub mod forge_manifest;
pub mod processor;

pub use library::*;
pub use argument::*;
pub use asset::*;
pub use rule::*;
pub use vanilla_manifest::*;

pub trait Allowed {
    fn allowed(&self) -> bool;
}