use bevy::{app::PluginGroupBuilder, prelude::*};

mod recipes;
pub use recipes::Recipe;

pub struct AssetPlugins;

impl PluginGroup for AssetPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(recipes::PZRecipesAssetPlugin);
    }
}
