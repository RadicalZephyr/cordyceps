use bevy::{asset::LoadedFolder, prelude::*};
use cordyceps::assets::Recipe;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(cordyceps::AssetPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, print_on_load)
        .run();
}

#[derive(Default, Resource)]
struct State {
    folder: Handle<LoadedFolder>,
    printed: bool,
}

fn setup(mut state: ResMut<State>, asset_server: Res<AssetServer>) {
    let recipes = asset_server.load_folder("");
    state.folder = recipes;
}

fn print_on_load(
    mut state: ResMut<State>,
    folders: ResMut<Assets<LoadedFolder>>,
    recipes: ResMut<Assets<Recipe>>,
) {
    if state.printed || recipes.is_empty() {
        return;
    }
    if let Some(folder) = folders.get(&state.folder) {
        let recipes: Vec<_> = folder
            .handles
            .iter()
            .filter_map(|handle| recipes.get(&handle.clone_weak().typed::<Recipe>()))
            .collect();

        info!("Custom asset loaded: {:?}", recipes);
        state.printed = true;
    }
}
