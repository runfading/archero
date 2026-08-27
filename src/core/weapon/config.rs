use crate::core::attack::AttackSpec;
use crate::core::weapon::{TargetingMode, WeaponId};
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Asset, TypePath, Deserialize, Debug)]
pub struct WeaponConfig {
    pub id: WeaponId,
    pub targeting: TargetingMode,
    pub attack: AttackSpec,
}
