use bevy::prelude::*;

mod plugins;
use plugins::square_grid_test::GridPlugin;

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera_setup)
        .add_plugin(GridPlugin)
        .run();
}
