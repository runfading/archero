mod actors;
mod asset;
mod core;
mod font;
mod ui;
mod world;

use crate::asset::AssetLoadingPlugin;
use crate::core::CorePlugin;
use crate::font::FontPlugin;
use crate::ui::UiPlugin;
use crate::world::WorldPlugin;
use avian2d::{
    PhysicsPlugins,
    prelude::{Gravity, PhysicsTime},
};
use bevy::prelude::*;
use serde::Deserialize;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    StartupLoading,
    MainMenu,
    GameLoading,
    InGame,
    GameOver,
    GameClear,
    AssetLoadingError,
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

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RunSet {
    Playing,
}

#[derive(Deserialize)]
pub struct StartUpConfig {
    pub title: String,
    pub min_window_width: f32,
    pub min_window_height: f32,
    pub resolution: (u32, u32),
}

impl StartUpConfig {
    /// 加载启动配置
    pub fn load() -> Self {
        ron::from_str(include_str!("../assets/config/start_config.ron"))
            .expect("启动配置读取错误！")
    }
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
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity::ZERO)
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
        .configure_sets(
            Update,
            (RunSet::Playing
                .run_if(in_state(GameState::InGame).and_then(in_state(RunPhase::Playing))),),
        )
        .add_plugins(AssetLoadingPlugin)
        .add_plugins(FontPlugin)
        .init_state::<GameState>()
        .add_sub_state::<RunPhase>()
        .add_plugins(CorePlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(UiPlugin)
        .run();
}

fn spawn_world(mut commands: Commands) {
    commands.spawn(Camera2d);
}
