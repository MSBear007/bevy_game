use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    Grid,
}

#[derive(Component)]
pub struct IsGridEntity;

#[derive(Component)]
pub struct IsMenuEntity;
