use bevy::prelude::*;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 10;
pub const DEFAULT_STATE: bool = false;

#[derive(Default)]
pub struct GridAssets {
    pub dead: Handle<Image>,
    pub alive: Handle<Image>,
}

#[derive(Component)]
pub struct Cell {
    pub is_alive: bool,
}

#[derive(Component)]
pub struct Coordinates {
    pub i: usize,
    pub j: usize,
}
