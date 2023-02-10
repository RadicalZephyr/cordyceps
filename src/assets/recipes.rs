use bevy::{asset::AssetLoader, prelude::*, reflect::TypeUuid};
use serde::Deserialize;

pub struct PZRecipesAssetPlugin;

impl Plugin for PZRecipesAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Recipe>();
        app.init_asset_loader::<RecipeLoader>();
    }
}

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "34aea543-3fec-401a-ae19-1e42f280d51c"]
pub struct Recipe {
    name: String,
}

#[derive(Default)]
pub struct RecipeLoader;

impl AssetLoader for RecipeLoader {
    fn load<'a>(
        &'a self,
        _bytes: &'a [u8],
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {})
    }

    fn extensions(&self) -> &[&str] {
        todo!("recipes extensions")
    }
}
