use serde::Deserialize;

use crate::{Argument, Libraries, VersionTrait};

#[derive(Deserialize)]
pub struct Arguments {
    pub jvm: Option<Vec<Argument>>,
    pub game: Vec<Argument>,
}

#[derive(Deserialize)]
pub struct FabricVersion {
    #[serde(rename = "mainClass")]
    pub main_class: String,
    pub libraries: Libraries,
    pub arguments: Arguments,
}

impl VersionTrait for FabricVersion {
    fn join(&self, vanilla: &mut crate::version::Version) {
        vanilla.main_class = self.main_class.clone();
        vanilla.libraries.extend(self.libraries.clone());
        if let Some(arguments) = &mut vanilla.arguments {
            arguments.game.extend(arguments.game.clone());
            if let Some(jvm) = &self.arguments.jvm {
                arguments.jvm.extend(jvm.clone());
            }
        }
    }
}