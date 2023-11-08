use bevy::{asset::AssetLoader, prelude::*, reflect::TypeUuid};
use futures_lite::AsyncRead;
use serde::Deserialize;

pub struct PZScriptAssetsPlugin;

impl Plugin for PZScriptAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(ScriptLoader)
            .init_asset::<Script>();
        app.init_asset_loader::<ScriptLoader>();
    }
}

#[derive(Asset, Debug, Deserialize, TypePath, TypeUuid)]
#[uuid = "34aea543-3fec-401a-ae19-1e42f280d51c"]
pub struct Script {
    #[allow(dead_code)]
    name: String,
}

#[derive(Default)]
pub struct ScriptLoader;

#[derive(Copy, Clone, Debug, thiserror::Error)]
pub enum Error {}

impl AssetLoader for ScriptLoader {
    type Asset = Script;
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
