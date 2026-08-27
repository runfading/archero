use crate::actors::enemies::{Enemy, EnemyId, spawn_enemy};
use crate::asset::{GameAssets, GameMeshAssets};
use crate::core::weapon::config::WeaponConfigsParam;
use crate::world::level::config::{LevelConfig, SpawnBatchConfig, WaveConfig};
use crate::world::level::director;
use crate::world::level::spawn::spawn_wave_room;
use crate::{GameSet, GameState, RunPhase};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::RngExt;
use rand::prelude::ThreadRng;

#[derive(Resource)]
pub struct LevelDirector {
    /// 波次位置
    pub wave_index: usize,
    /// 波次以生产批
    pub batch_index: usize,
    /// 所有波次是否已经生成完毕；场上敌人清空后即可通关。
    pub all_waves_spawned: bool,
    /// 波次超时计时器
    pub wave_timer: Timer,
    /// 波次最后一次生产计时器
    pub spawn_timer: Timer,
}

pub struct LevelDirectorPlugin;
impl Plugin for LevelDirectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (enemies_num_game_loop, timer_level_game_loop, complete_level)
                .chain()
                .run_if(in_state(GameState::InGame).and_then(in_state(RunPhase::Playing)))
                .in_set(GameSet::Core),
        )
        .add_systems(OnExit(GameState::InGame), cleanup_level);
    }
}

/// 是否需要生成下一批次，下一波敌人。
fn timer_level_game_loop(
    mut commands: Commands,
    mut director: Option<ResMut<LevelDirector>>,
    timer: Res<Time>,
    level_config: Res<Assets<LevelConfig>>,
    game_assets: Res<GameAssets>,
    game_mesh_assets: Res<GameMeshAssets>,
    weapon_configs: WeaponConfigsParam,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(mut director) = director.as_mut() else {
        warn!("无关卡上下文");
        return;
    };

    if director.all_waves_spawned {
        return;
    }

    let Some(level_config) = game_assets.level_001_config(&level_config) else {
        warn!("无关卡配置");
        return;
    };

    let Ok(window) = window.single() else {
        error!("找不到窗口");
        return;
    };

    director.spawn_timer.tick(timer.delta());
    director.wave_timer.tick(timer.delta());

    // 这里若是波次超时，batch就没有了，后面看怎么搞，现在先不管
    if director.spawn_timer.just_finished() {
        if let Some(wave_config) = level_config.waves.get(director.wave_index) {
            director.batch_index += 1;

            let spawn_timer = spawn_wave_room(
                &mut commands,
                &game_mesh_assets,
                weapon_configs.all(),
                wave_config,
                director.batch_index,
                window.width(),
                window.height(),
            );

            if let Some(timer) = spawn_timer {
                director.spawn_timer = timer;
            } else {
                if let Some(timer) = spawn_timer {
                    director.spawn_timer = timer;
                } else if !director.wave_timer.is_finished() {
                    // 没有下一批次计时器那么说明已经是最后一批敌人，主动触发下一波敌人
                    director.wave_timer.finish();
                }
            }
        } else {
            error!(
                "波次信息有问题,索引越界:current{},max_len:{}",
                director.wave_index,
                level_config.waves.len()
            );
            return;
        }
    }

    if director.wave_timer.just_finished() {
        director.wave_index += 1;
        director.batch_index = 0;

        if let Some(wave_config) = level_config.waves.get(director.wave_index) {
            director.wave_timer = Timer::from_seconds(wave_config.max_time, TimerMode::Once);
            spawn_wave_room(
                &mut commands,
                &game_mesh_assets,
                weapon_configs.all(),
                wave_config,
                0,
                window.width(),
                window.height(),
            );
        } else {
            info!("关卡已经生成完成");
            director.all_waves_spawned = true;
        }
    }
}

/// 敌人被消灭时进入自动进入下一波敌人
fn enemies_num_game_loop(mut director: Option<ResMut<LevelDirector>>, enemies: Query<&Enemy>) {
    let Some(mut director) = director.as_mut() else {
        warn!("无关卡上下文");
        return;
    };

    if director.all_waves_spawned {
        return;
    }

    if enemies.iter().len() == 0 {
        director.spawn_timer.almost_finish();
    }
}

/// 所有波次生成完毕且最后一批敌人被消灭后进入通关结算。
fn complete_level(
    director: Option<Res<LevelDirector>>,
    enemies: Query<(), With<Enemy>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(director) = director else {
        return;
    };

    if director.all_waves_spawned && enemies.is_empty() {
        next_state.set(GameState::GameClear);
    }
}

fn cleanup_level(mut commands: Commands) {
    commands.remove_resource::<LevelDirector>();
}
