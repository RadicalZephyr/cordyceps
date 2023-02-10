use bevy::{asset::AssetServerSettings, prelude::*};
use cordyceps::assets::Recipe;

fn main() {
    App::new()
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(cordyceps::AssetPlugins)
        .add_startup_system(setup)
        .add_system(print_on_load)
        .run();
}

#[derive(Default)]
struct State {
    handles: Vec<HandleUntyped>,
    printed: bool,
}

fn setup(mut state: ResMut<State>, asset_server: Res<AssetServer>) {
    let recipes = asset_server
        .load_folder("")
        .expect("failed to load recipes");
    state.handles = recipes;
}

fn print_on_load(mut state: ResMut<State>, recipes: ResMut<Assets<Recipe>>) {
    if state.printed || recipes.is_empty() {
        return;
    }
    let recipes: Vec<_> = state
        .handles
        .iter()
        .filter_map(|handle| recipes.get(&handle.typed_weak::<Recipe>()))
        .collect();

    info!("Custom asset loaded: {:?}", recipes);
    state.printed = true;
}
