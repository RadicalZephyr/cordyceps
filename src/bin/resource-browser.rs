use bevy::{asset::AssetServerSettings, prelude::*};

fn main() {
    App::new()
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(cordyceps::AssetPlugins)
        .run();
}
