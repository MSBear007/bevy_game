use std::env;

use bevy::prelude::*;

mod plugins;
use plugins::debug::DebugPlugin;
use plugins::main_menu::plugin::MainMenuPlugin;
use plugins::square_grid::plugin::GridPlugin;
use plugins::states::AppState;

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut app = App::new();
    for i in 1..args.len() {
        if args[i] == "debug" {
            app.insert_resource(bevy::log::LogSettings {
                level: bevy::log::Level::DEBUG,
                ..default()
            });
        }
    }
    app.add_plugins(DefaultPlugins)
        .add_startup_system(camera_setup)
        .add_state(AppState::Grid)
        .add_plugin(DebugPlugin)
        .add_plugin(GridPlugin)
        .add_plugin(MainMenuPlugin)
        .run();
}
