use bevy::prelude::*;

pub fn mouse_to_world_position(window: &Window, mouse_pos: Vec2) -> Vec2 {
    Vec2::new(
        mouse_pos[0] - window.width()/2.,
        mouse_pos[1] - window.height()/2.
    )

}