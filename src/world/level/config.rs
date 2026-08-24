use crate::actors::enemies::EnemyId;
use bevy::asset::Asset;
use bevy::prelude::TypePath;
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
    pub enemy_id: EnemyId,
    pub count: u32,
}
