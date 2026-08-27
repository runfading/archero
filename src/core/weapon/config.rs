use crate::asset::GameAssets;
use crate::core::attack::AttackSpec;
use crate::core::weapon::{TargetingMode, WeaponId};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Asset, TypePath, Deserialize, Debug)]
pub struct WeaponConfig {
    /// id
    pub id: WeaponId,
    /// 目标选择配置
    pub targeting: TargetingMode,
    /// 攻击方式配置
    pub attack: AttackSpec,
    /// 基础倍率
    pub base_multiplying_power: f32,
}

#[derive(Asset, TypePath, Deserialize, Debug)]
#[serde(transparent)]
pub struct WeaponConfigs(pub HashMap<WeaponId, WeaponConfig>);

impl WeaponConfigs {
    pub fn get(&self, id: WeaponId) -> Option<&WeaponConfig> {
        self.0.get(&id)
    }
}

#[derive(SystemParam)]
pub struct WeaponConfigsParam<'w> {
    game_assets: Res<'w, GameAssets>,
    configs: Res<'w, Assets<WeaponConfigs>>,
}

impl WeaponConfigsParam<'_> {
    pub fn get(&self, id: WeaponId) -> &WeaponConfig {
        self.game_assets.get_weapon_config(id, &self.configs)
    }
}
