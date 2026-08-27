use crate::asset::GameAssets;
use bevy::asset::Asset;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Asset, TypePath, Deserialize, Debug)]
pub struct PlayerConfig {
    /// 移动速度
    pub move_speed: f32,
    /// 基础血量
    pub base_hp: f32,
    /// 基础倍率
    pub base_multiplying_power: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            move_speed: 250.0,
            base_hp: 100.0,
            base_multiplying_power: 1.0,
        }
    }
}

#[derive(SystemParam)]
pub struct PlayerConfigParam<'w> {
    game_assets: Res<'w, GameAssets>,
    configs: Res<'w, Assets<PlayerConfig>>,
}

impl PlayerConfigParam<'_> {
    pub fn get(&self) -> &PlayerConfig {
        self.game_assets.player_config(&self.configs)
    }
}
