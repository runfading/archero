use crate::actors::ActorsPlugin;
use crate::actors::enemies::spawn_enemy;
use crate::actors::player::config::PlayerConfig;
use crate::actors::player::spawn_player;
use crate::asset::{GameAssets, GameMeshAssets};
use crate::world::level::config::LevelConfig;
use crate::world::level::spawn::spawn_level;
use crate::{GameSet, GameState, RunPhase};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub mod attack;
pub mod health;

/// 局内单位标记
#[derive(Component, Default, Copy, Clone)]
pub struct RunEntity;

/// 单位阵营
#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Faction {
    Player,
    Enemy,
}

#[derive(Component, Default, Debug, Clone)]
pub struct MoveSpeed(pub f32);

#[derive(Resource, Default)]
pub struct RunStats {
    pub gold: usize,
    pub kills: usize,
    pub _time: f32,
}

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RunStats>()
            .add_plugins(ActorsPlugin)
            .add_systems(Startup, crate::spawn_world)
            .add_systems(
                OnEnter(GameState::InGame),
                setup_run.in_set(GameSet::Gameplay),
            )
            .add_systems(
                OnExit(GameState::InGame),
                teardown_run.in_set(GameSet::Gameplay),
            );
    }
}

fn spawn_world(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_run(
    mut commands: Commands,
    mut stats: ResMut<RunStats>,
    asset: Res<GameMeshAssets>,
    player_config: Res<Assets<PlayerConfig>>,
    game_assets: Res<GameAssets>,
    mut next_phase: ResMut<NextState<RunPhase>>,
    level_config: Res<Assets<LevelConfig>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    *stats = RunStats::default();
    next_phase.set(RunPhase::Playing);
    let player_config = if let Some(player_config) = game_assets.player_config(&player_config) {
        player_config
    } else {
        warn!("没有找到玩家基础配置，使用默认值");
        &PlayerConfig::default()
    };

    let Some(level_config) = game_assets.level_001_config(&level_config) else {
        warn!("无关卡配置");
        return;
    };

    let Ok(window) = window.single() else {
        error!("找不到窗口");
        return;
    };

    spawn_player(&mut commands, &asset, player_config);
    spawn_level(&mut commands, &asset, level_config, window);
}

/// 清理游戏运行状态：
///
/// - 递归移除所有带有 [`RunEntity`] 标记的实体；
fn teardown_run(mut commands: Commands, entities: Query<Entity, With<RunEntity>>) {
    for e in &entities {
        commands.entity(e).despawn();
    }
}
