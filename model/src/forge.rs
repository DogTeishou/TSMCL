use serde::{Deserialize};

use crate::{Arguments, Libraries, VersionTrait, version::Version};

#[derive(Deserialize)]
pub struct ForgeVersion {
    #[serde(rename = "mainClass")]
    pub main_class: String,
    pub libraries: Libraries,
    pub arguments: Option<Arguments>,
    #[serde(rename = "minecraftArguments")]
    pub minecraft_arguments: Option<String>,
}

impl VersionTrait for ForgeVersion {
    fn join(&self, vanilla: &mut Version) {
        vanilla.main_class = self.main_class.clone();
        vanilla.libraries.extend(self.libraries.clone());
        if let Some(arguments) = &mut vanilla.arguments {
            if let Some(args) = &self.arguments {
                arguments.jvm.extend(args.jvm.clone());
                arguments.game.extend(args.game.clone());
            }
        }
        if let Some(mc_args) = &mut vanilla.minecraft_arguments {
            if let Some(args) = &self.minecraft_arguments {
                *mc_args = args.clone();
            }
        }
    }
}