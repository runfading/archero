pub mod contact_attack;
pub mod projectile_attack;

use crate::core::RunEntity;
use crate::core::attack::contact_attack::update_knockback;
use crate::core::attack::projectile_attack::ProjectilePlugin;
use crate::{GameSet, RunSet};
use avian2d::prelude::LinearVelocity;
use bevy::prelude::Component;
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component, Debug, Clone, Deserialize)]
#[require(RunEntity)]
pub enum AttackSpec {
    /// 近战
    Melee {
        #[serde(default = "AttackSpec::default_melee_range")]
        range: f32,
        #[serde(default = "AttackSpec::default_melee_cooldown")]
        cooldown: f32,
        #[serde(default = "AttackSpec::default_melee_effect")]
        effect: CombatEffect,
    },
    /// 投掷物/发射物
    Projectile {
        #[serde(default = "AttackSpec::default_projectile_range")]
        range: f32,
        #[serde(default = "AttackSpec::default_projectile_cooldown")]
        cooldown: f32,
        #[serde(default = "AttackSpec::default_projectile_speed")]
        projectile_speed: f32,
        #[serde(default = "AttackSpec::default_projectile_effect")]
        effect: CombatEffect,
    },
    /// 法术/技能
    Spell {
        #[serde(default = "AttackSpec::default_spell_radius")]
        radius: f32,
        #[serde(default = "AttackSpec::default_spell_cooldown")]
        cooldown: f32,
        #[serde(default = "AttackSpec::default_spell_effect")]
        effect: CombatEffect,
    },
    /// 治疗
    Heal {
        #[serde(default = "AttackSpec::default_heal_radius")]
        radius: f32,
        #[serde(default = "AttackSpec::default_heal_cooldown")]
        cooldown: f32,
        #[serde(default = "AttackSpec::default_heal_effect")]
        effect: CombatEffect,
    },
}

/// default config
impl AttackSpec {
    fn default_melee_effect() -> CombatEffect {
        CombatEffect::Damage { amount: 1.0 }
    }

    fn default_melee_range() -> f32 {
        10.0
    }

    fn default_melee_cooldown() -> f32 {
        1.0
    }

    fn default_projectile_effect() -> CombatEffect {
        CombatEffect::Damage { amount: 4.0 }
    }

    fn default_projectile_range() -> f32 {
        50.0
    }

    fn default_projectile_cooldown() -> f32 {
        1.0
    }

    fn default_projectile_speed() -> f32 {
        120.0
    }

    fn default_spell_effect() -> CombatEffect {
        CombatEffect::Damage { amount: 4.0 }
    }

    fn default_spell_radius() -> f32 {
        50.0
    }

    fn default_spell_cooldown() -> f32 {
        2.0
    }

    fn default_heal_effect() -> CombatEffect {
        CombatEffect::Heal { amount: 4.0 }
    }

    fn default_heal_radius() -> f32 {
        50.0
    }

    fn default_heal_cooldown() -> f32 {
        2.0
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum CombatEffect {
    Damage { amount: f32 },
    Heal { amount: f32 },
}

/// 攻击范围
#[derive(Component, Default, Debug, Clone)]
pub struct AttackTriggerRange {
    pub range: f32,
}

/// 单位正处于击退状态。存在该组件时，常规移动控制暂时让出速度控制权。
#[derive(Component, Debug)]
pub struct Knockback(Timer);

impl Knockback {
    fn new(duration: f32) -> Self {
        Self(Timer::from_seconds(duration, TimerMode::Once))
    }
}

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProjectilePlugin).add_systems(
            Update,
            update_knockback
                .in_set(GameSet::Core)
                .in_set(RunSet::Playing),
        );
    }
}

/// 击退组件，需要其他实体配合查询时使用[`Without<Knockback>`]
///
/// 到期后交还速度控制权给玩家输入或敌人 AI。
pub fn update_knockback(
    mut commands: Commands,
    time: Res<Time>,
    mut entities: Query<(Entity, &mut Knockback, &mut LinearVelocity)>,
) {
    for (entity, mut knockback, mut velocity) in &mut entities {
        if knockback.0.tick(time.delta()).just_finished() {
            velocity.0 = Vec2::ZERO;
            commands.entity(entity).remove::<Knockback>();
        }
    }
}
