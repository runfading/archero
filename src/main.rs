pub mod config;
mod font;
mod game;
mod menu;

use crate::config::StartUpConfig;
use crate::font::FontPlugin;
use crate::game::GamePlugin;
use crate::menu::MenuPlugin;
use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

#[derive(SubStates, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
#[source(GameState = GameState::InGame)]
pub enum RunPhase {
    #[default]
    Playing,
    LevelUp,
    Paused,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSet {
    Core,
    Gameplay,
    Ui,
}

fn main() {
    let start_up_config = StartUpConfig::load();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: start_up_config.title,
                resolution: start_up_config.resolution.into(),
                resize_constraints: WindowResizeConstraints {
                    min_width: start_up_config.min_window_width,
                    min_height: start_up_config.min_window_height,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .configure_sets(
            Update,
            (
                GameSet::Core,
                GameSet::Gameplay.after(GameSet::Core),
                GameSet::Ui.after(GameSet::Gameplay),
            ),
        )
        .configure_sets(
            OnEnter(GameState::InGame),
            (GameSet::Gameplay, GameSet::Ui.after(GameSet::Gameplay)),
        )
        .configure_sets(
            OnExit(GameState::InGame),
            (GameSet::Ui, GameSet::Gameplay.after(GameSet::Ui)),
        )
        .add_plugins(FontPlugin)
        .init_state::<GameState>()
        .add_sub_state::<RunPhase>()
        .add_systems(Startup, spawn_world)
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        .run();
}

fn spawn_world(mut commands: Commands) {
    commands.spawn(Camera2d);
}
