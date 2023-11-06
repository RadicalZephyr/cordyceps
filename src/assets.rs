use bevy::{app::PluginGroupBuilder, prelude::*};

mod recipes;
pub use recipes::Recipe;

pub struct AssetPlugins;

impl PluginGroup for AssetPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(recipes::PZRecipesAssetPlugin)
    }
}
