use crate::actors::ActorsPlugin;
use crate::actors::enemies::spawn_enemy;
use crate::actors::player::config::{PlayerConfig, PlayerConfigParam};
use crate::actors::player::spawn_player;
use crate::asset::{GameAssets, GameMeshAssets};
use crate::core::attack::AttackPlugin;
use crate::core::health::HealthPlugin;
use crate::core::pause::PausePlugin;
use crate::core::weapon::config::{WeaponConfigs, WeaponConfigsParam};
use crate::core::weapon::{WeaponId, WeaponPlugin};
use crate::world::level::config::{LevelConfig, LevelConfigParam};
use crate::world::level::spawn::spawn_level;
use crate::{GameSet, GameState, RunPhase};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub mod attack;
pub mod health;
mod hit;
mod pause;
pub mod weapon;

/// 局内单位标记
#[derive(Component, Default, Copy, Clone)]
pub struct RunEntity;

/// 单位阵营
#[derive(Component, Default, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Faction {
    #[default]
    Player,
    Enemy,
}

#[derive(Component, Default, Debug, Clone)]
pub struct MoveSpeed(pub f32);

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
            .add_plugins(PausePlugin)
            .add_plugins(ActorsPlugin)
            .add_plugins(AttackPlugin)
            .add_plugins(WeaponPlugin)
            .add_plugins(HealthPlugin)
            .add_systems(Startup, crate::spawn_world)
            .add_systems(
                OnEnter(GameState::InGame),
                setup_run.in_set(GameSet::Gameplay),
            )
            .add_systems(
                OnExit(GameState::InGame),
                teardown_run.in_set(GameSet::Gameplay),
            )
            .add_systems(
                Update,
                update_run_time
                    .run_if(in_state(GameState::InGame).and_then(in_state(RunPhase::Playing)))
                    .in_set(GameSet::Core),
            );
    }
}

fn update_run_time(time: Res<Time>, mut stats: ResMut<RunStats>) {
    stats.time += time.delta_secs();
}

fn spawn_world(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_run(
    mut commands: Commands,
    mut stats: ResMut<RunStats>,
    asset: Res<GameMeshAssets>,
    mut next_phase: ResMut<NextState<RunPhase>>,
    window: Query<&Window, With<PrimaryWindow>>,
    player_config: PlayerConfigParam,
    level_config: LevelConfigParam,
    weapon_asset: WeaponConfigsParam,
) {
    *stats = RunStats::default();
    next_phase.set(RunPhase::Playing);

    let player_config = player_config.get();
    let level_config = level_config.get();

    let Ok(window) = window.single() else {
        error!("找不到窗口");
        return;
    };

    spawn_player(
        &mut commands,
        &asset,
        weapon_asset.get(WeaponId::Bow),
        player_config,
    );

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
