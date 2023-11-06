use bevy::{asset::AssetLoader, prelude::*, reflect::TypeUuid};
use futures_lite::AsyncRead;
use serde::Deserialize;

pub struct PZRecipesAssetPlugin;

impl Plugin for PZRecipesAssetPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(RecipeLoader)
            .init_asset::<Recipe>();
        app.init_asset_loader::<RecipeLoader>();
    }
}

#[derive(Asset, Debug, Deserialize, TypePath, TypeUuid)]
#[uuid = "34aea543-3fec-401a-ae19-1e42f280d51c"]
pub struct Recipe {
    name: String,
}

#[derive(Default)]
pub struct RecipeLoader;

#[derive(Copy, Clone, Debug, thiserror::Error)]
pub enum Error {}

impl AssetLoader for RecipeLoader {
    type Asset = Recipe;
    type Settings = ();
    type Error = Error;

    fn load<'a>(
        &'a self,
        _reader: &'a mut (dyn AsyncRead + Sync + Unpin + Send + '_),
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move { todo!() })
    }

    fn extensions(&self) -> &[&str] {
        todo!("recipes extensions")
    }
}
