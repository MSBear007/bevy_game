use super::super::debug::*;
use super::super::states::*;
use super::super::util::*;
use super::components::*;
use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GridAssets::default())
            .add_system_set(SystemSet::on_enter(AppState::Grid).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::Grid)
                    .with_system(press_on_cell)
                    .with_system(exit_grid),
            )
            .add_system_set(SystemSet::on_exit(AppState::Grid).with_system(despawn::<IsGridEntity>))
            .add_system_set(
                SystemSet::on_update(DebugState::DebugState)
                    .with_run_criteria(is_grid_view)
                    .with_system(debug_changed_cells),
            );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle_alive: Handle<Image> = asset_server.load("sprites/alive.png");
    let handle_dead: Handle<Image> = asset_server.load("sprites/dead.png");
    commands.insert_resource(GridAssets {
        dead: handle_dead,
        alive: handle_alive,
    });
    let x_offset: f32 = -(WIDTH as f32 * 16.0) / 2.0;
    let y_offset: f32 = (HEIGHT as f32 * 16.0) / 2.0;
    //grid
    for i in 0..10 {
        for j in 0..10 {
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_xyz(
                        x_offset + 16.0 * ((i % WIDTH) as f32),
                        16.0 * (j as f32) - y_offset,
                        0.,
                    ),
                    texture: match DEFAULT_STATE {
                        true => asset_server.load("sprites/alive.png"),
                        false => asset_server.load("sprites/dead.png"),
                    },
                    ..default()
                })
                .insert(Cell {
                    is_alive: DEFAULT_STATE,
                })
                .insert(Coordinates { i, j })
                .insert(IsGridEntity);
        }
    }
}

fn debug_changed_cells(changed_cells: Query<(&Cell, &Coordinates), Changed<Cell>>) {
    use bevy::log::*;
    for (cell, coordinates) in changed_cells.iter() {
        debug!(
            "Cell {} at {} {}",
            cell.is_alive, coordinates.i, coordinates.j
        );
    }
}

fn press_on_cell(
    windows: Res<Windows>,
    imgs: Res<GridAssets>,
    buttons: Res<Input<MouseButton>>,
    mut grid: Query<(&mut Cell, &mut Handle<Image>)>,
) {
    let window = windows.get_primary().unwrap();

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(pos) = window.cursor_position() {
            // calculate the position of cell
            let pos = mouse_to_world_position(window, pos);
            let x_offset: f32 = -(WIDTH as f32 * 16.0) / 2.0 - 8.0;
            let y_offset = (HEIGHT as f32 * 16.0) / 2.0 - 8.0;
            let i = (pos[0] - x_offset) as f32 / 16.0;
            let j = HEIGHT as f32 - (y_offset - pos[1]) as f32 / 16.0;
            let i = i as isize;
            let j = j as isize;
            // change state of cell
            if i < WIDTH as isize && j < HEIGHT as isize && i >= 0 && j >= 0 {
                for (index, (mut cell, mut img)) in grid.iter_mut().enumerate() {
                    if index == WIDTH * i as usize + j as usize {
                        cell.is_alive = !cell.is_alive;
                        if *img == imgs.alive {
                            *img = imgs.dead.clone()
                        } else if *img == imgs.dead {
                            *img = imgs.alive.clone()
                        }
                    }
                }
            }
        }
    }
}

fn exit_grid(mut state: ResMut<State<AppState>>, key: Res<Input<KeyCode>>) {
    if key.just_released(KeyCode::Escape) {
        match state.set(AppState::MainMenu) {
            Ok(_) => {}
            Err(e) => bevy::log::error!("Failed to change app state due of {}", e),
        }
    }
}

fn is_grid_view(state: Res<State<DebugState>>) -> ShouldRun {
    if state.current() == &DebugState::DebugState {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
