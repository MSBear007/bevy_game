use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DebugState {
    DebugState,
    NormalState
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(DebugState::NormalState)
        .add_system(debug_mode);
    }
}

fn debug_mode(mut state: ResMut<State<DebugState>>, key: Res<Input<KeyCode>>) {
    if key.just_released(KeyCode::D) {
        match state.current() {
            DebugState::DebugState => {
                state.set(DebugState::NormalState).unwrap();
            },
            DebugState::NormalState => {
                state.set(DebugState::DebugState).unwrap();
            }
        }
    }
}