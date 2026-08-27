use crate::actors::enemies::EnemyId;
use crate::actors::enemies::config::EnemyConfig;
use crate::asset::GameAssets;
use bevy::asset::Asset;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use serde::Deserialize;

/// 关卡配置
#[derive(Asset, TypePath, Deserialize, Debug)]
pub struct LevelConfig {
    pub waves: Vec<WaveConfig>,
}

/// 波次配置
#[derive(Deserialize, Debug)]
pub struct WaveConfig {
    pub batches: Vec<SpawnBatchConfig>,
    /// 本波次最长清场时间
    pub max_time: f32,
}

/// 敌人生成批次配置
#[derive(Deserialize, Debug)]
pub struct SpawnBatchConfig {
    pub config: Vec<EnemyBatchConfig>,
    pub next_batch_delay: f32,
}

/// 生成敌人配置
#[derive(Deserialize, Debug)]
pub struct EnemyBatchConfig {
    pub enemy_config: EnemyConfig,
    pub count: u32,
}

#[derive(SystemParam)]
pub struct LevelConfigParam<'w> {
    game_assets: Res<'w, GameAssets>,
    configs: Res<'w, Assets<LevelConfig>>,
}

impl LevelConfigParam<'_> {
    pub fn get(&self) -> &LevelConfig {
        self.game_assets
            .level_001_config(&self.configs)
            .expect("没有找到关卡配置")
    }
}
