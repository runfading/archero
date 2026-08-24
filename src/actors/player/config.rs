use bevy::asset::Asset;
use bevy::prelude::TypePath;
use serde::Deserialize;

#[derive(Asset, TypePath, Deserialize, Debug)]
pub struct PlayerConfig {
    /// 移动速度
    pub move_speed: f32,
    /// 基础血量
    pub base_hp: f32,
    /// 伤害
    pub damage: f32,
    /// 攻击间隔
    pub attack_interval: f32,
    /// 攻击范围
    pub range: f32,
    /// 弹丸速度
    pub projectile_speed: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            move_speed: 250.0,
            base_hp: 100.0,
            damage: 10.0,
            attack_interval: 0.6,
            range: 340.0,
            projectile_speed: 720.0,
        }
    }
}