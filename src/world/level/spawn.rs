use crate::actors::enemies::config::EnemyConfig;
use crate::actors::enemies::{EnemyId, spawn_enemy};
use crate::asset::GameMeshAssets;
use crate::core::weapon::config::WeaponConfigs;
use crate::world::level::config::{LevelConfig, SpawnBatchConfig, WaveConfig};
use crate::world::level::director::LevelDirector;
use bevy::log::info;
use bevy::math::Vec2;
use bevy::prelude::*;
use rand::RngExt;
use rand::prelude::ThreadRng;

/// 初始化关卡敌人生成
pub fn spawn_level(
    mut commands: &mut Commands,
    assets: &GameMeshAssets,
    weapons: &WeaponConfigs,
    level_config: &LevelConfig,
    window: &Window,
) {
    let first_wave = if let Some(first_wave) = level_config.waves.get(0) {
        first_wave
    } else {
        warn!("无关卡配置");
        return;
    };

    if let Some(first_batch) = first_wave.batches.first() {
        commands.insert_resource(LevelDirector {
            wave_index: 0,
            batch_index: 0,
            all_waves_spawned: false,
            wave_timer: Timer::from_seconds(first_wave.max_time, TimerMode::Once),
            spawn_timer: Timer::from_seconds(first_batch.next_batch_delay, TimerMode::Once),
        });

        spawn_batch_room(
            commands,
            assets,
            weapons,
            first_batch,
            window.width(),
            window.height(),
        );
    } else {
        warn!("无批次配置");
    }
}

/// 生成指定波次的指定批次敌人。
/// 存在批次则返回下一批次生成的 [`Some(Timer)`]
/// 不存在该批次则返回[`None`]
pub fn spawn_wave_room(
    commands: &mut Commands,
    assets: &GameMeshAssets,
    weapons: &WeaponConfigs,
    batch_config: &WaveConfig,
    batch_index: usize,
    width: f32,
    height: f32,
) -> Option<Timer> {
    if let Some(batch_config) = batch_config.batches.get(batch_index) {
        spawn_batch_room(commands, assets, weapons, batch_config, width, height);
        Some(Timer::from_seconds(
            batch_config.next_batch_delay,
            TimerMode::Once,
        ))
    } else {
        info!("该波次已经生成完成");
        None
    }
}

/// 生成该批次敌人
pub fn spawn_batch_room(
    commands: &mut Commands,
    assets: &GameMeshAssets,
    weapons: &WeaponConfigs,
    batch_config: &SpawnBatchConfig,
    width: f32,
    height: f32,
) {
    let mut rng = rand::rng();

    let mut spawn = |config: &EnemyConfig, count: u32| {
        for _ in 0..count {
            let pos = random_edge_pos(&mut rng, width, height);
            spawn_enemy(commands, config, weapons, assets, pos)
        }
    };

    for enemy in batch_config.config.iter() {
        spawn(&enemy.enemy_config, enemy.count)
    }
}

/// 敌人随机位置系统
fn random_edge_pos(rng: &mut ThreadRng, width: f32, height: f32) -> Vec2 {
    let half_width = width / 2.0;
    let half_height = height / 2.0;

    let edge = rng.random_range::<u32, _>(..4);
    let x = rng.random_range(-half_width + 40.0..half_width - 40.0);
    let y = rng.random_range(-half_height + 40.0..half_height - 40.0);
    match edge {
        0 => Vec2::new(x, -half_height + 40.0),
        1 => Vec2::new(x, half_height - 40.0),
        2 => Vec2::new(-half_width + 40.0, y),
        _ => Vec2::new(half_width - 40.0, y),
    }
}
