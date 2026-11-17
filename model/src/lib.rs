pub mod version;
pub mod argument;
pub mod rule;
pub mod library;
pub mod asset;
pub mod vanilla_manifest;
pub mod forge_manifest;
pub mod forge;
pub mod processor;
pub mod fabric;

pub use library::*;
pub use argument::*;
pub use asset::*;
pub use rule::*;
pub use vanilla_manifest::*;

use crate::version::Version;

pub trait Allowed {
    fn allowed(&self) -> bool;
}

pub trait VersionTrait {
    fn join(&self, vanilla: &mut Version);
}