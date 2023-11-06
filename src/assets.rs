use bevy::{app::PluginGroupBuilder, prelude::*};

mod script;
pub use script::Script;

pub struct AssetPlugins;

impl PluginGroup for AssetPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(script::PZScriptAssetsPlugin)
    }
}
