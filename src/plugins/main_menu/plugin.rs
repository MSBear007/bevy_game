use super::super::states::*;
use super::super::util::*;
use super::components::*;
use bevy::prelude::*;
use bevy::window::WindowResized;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MainMenuAssets::default())
            .add_event::<WindowResized>()
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup))
            .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(click_on))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(despawn::<IsMenuEntity>));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = MainMenuAssets {
        play_button: asset_server.load("buttons/new_game.png"),
        to_grid_button: asset_server.load("buttons/play_grid.png"),
        font: asset_server.load("fonts/Quicksand-Regular.otf"),
        background_image: asset_server.load("background-image.png"),
    };
    commands.insert_resource(assets.clone());
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(IsMenuEntity);
    commands
        .spawn_bundle(background_image(&assets.background_image))
        .with_children(|parent| {
            parent.spawn_bundle(main_text(&assets.font));
        })
        .with_children(|parent| {
            parent.spawn_bundle(menu_button(&assets.play_button));
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(menu_button(&assets.to_grid_button))
                .insert(GridButton);
        })
        .insert(IsMenuEntity);
}

fn background_image(asset: &Handle<Image>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
            },
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::Center,
            ..default()
        },
        image: asset.clone().into(),
        ..default()
    }
}

fn menu_button(asset: &Handle<Image>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: Size::new(Val::Px(400.0), Val::Px(100.0)),
            border: Rect {
                left: Val::Px(5.0),
                right: Val::Px(5.0),
                bottom: Val::Px(5.0),
                top: Val::Px(5.0),
            },
            margin: Rect::all(Val::Auto),
            ..default()
        },
        image: asset.clone().into(),
        ..default()
    }
}

fn main_text(asset: &Handle<Font>) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            "My Lovely Game",
            TextStyle {
                color: Color::Rgba {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                    alpha: 1.0,
                },
                font_size: 64.0,
                font: asset.clone(),
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        style: Style {
            margin: Rect::all(Val::Auto),
            align_self: AlignSelf::Center,
            ..default()
        },
        ..default()
    }
}

fn click_on(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<GridButton>)>,
    mut state: ResMut<State<AppState>>
) {
    for interaction in query.iter() {
        match *interaction {
            Interaction::Clicked => state.set(AppState::Grid).unwrap(),
            _ => {}
        }
    }
}