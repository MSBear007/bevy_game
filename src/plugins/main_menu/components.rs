use bevy::prelude::*;

#[derive(Default, Clone)]
pub struct MainMenuAssets {
    pub play_button: Handle<Image>,
    pub to_grid_button: Handle<Image>,
    pub font: Handle<Font>,
    pub background_image: Handle<Image>,
}

#[derive(Component)]
pub struct GridButton;
