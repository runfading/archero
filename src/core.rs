use crate::actors::player::{PlayerPlugin, spawn_player};
use crate::asset::{GameAssets, GameMeshAssets};
use crate::config::PlayerConfig;
use crate::{GameSet, GameState, RunPhase};
use bevy::prelude::*;

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

#[derive(Resource, Default)]
pub struct RunStats {
    pub gold: usize,
    pub kills: usize,
    pub time: f32,
}

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RunStats>()
            .add_systems(Startup, crate::spawn_world)
            .add_systems(
                OnEnter(GameState::InGame),
                setup_run.in_set(GameSet::Gameplay),
            )
            .add_systems(
                OnExit(GameState::InGame),
                teardown_run.in_set(GameSet::Gameplay),
            )
            .add_plugins(PlayerPlugin);
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
) {
    *stats = RunStats::default();
    next_phase.set(RunPhase::Playing);
    let player_config = if let Some(player_config) = game_assets.player_config(&player_config) {
        player_config
    } else {
        warn!("没有找到玩家基础配置，使用默认值");
        &PlayerConfig::default()
    };

    init_run(&mut commands, &asset, player_config);
}

fn init_run(commands: &mut Commands, asset: &GameMeshAssets, player_config: &PlayerConfig) {
    spawn_player(commands, asset, player_config)
}

/// 清理游戏运行状态：
///
/// - 递归移除所有带有 [`RunEntity`] 标记的实体；
fn teardown_run(mut commands: Commands, entities: Query<Entity, With<RunEntity>>) {
    for e in &entities {
        commands.entity(e).despawn();
    }
}
