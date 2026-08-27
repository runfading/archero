pub mod contact_attack;
pub mod projectile_attack;

use crate::core::RunEntity;
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
    Melee(MeleeAttackProperty),
    /// 投掷物/发射物
    Projectile(ProjectileAttackProperty),
    /// 区域
    Area(AreaAttackProperty),
    /// 光束
    Beam(BeamAttackProperty),
}

#[derive(Component, Debug, Clone, Deserialize)]
#[serde(default)]
pub struct MeleeAttackProperty {
    pub range: f32,
    pub cooldown: f32,
    pub effect: CombatEffect,
}

impl Default for MeleeAttackProperty {
    fn default() -> Self {
        Self {
            range: 10.0,
            cooldown: 1.0,
            effect: CombatEffect::Damage { amount: 1.0 },
        }
    }
}

#[derive(Component, Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ProjectileAttackProperty {
    pub range: f32,
    pub cooldown: f32,
    pub projectile_speed: f32,
    pub effect: CombatEffect,
}

impl Default for ProjectileAttackProperty {
    fn default() -> Self {
        Self {
            range: 50.0,
            cooldown: 1.0,
            projectile_speed: 120.0,
            effect: CombatEffect::Damage { amount: 4.0 },
        }
    }
}

#[derive(Component, Debug, Clone, Deserialize)]
#[serde(default)]
pub struct AreaAttackProperty {
    pub radius: f32,
    pub cooldown: f32,
    pub effect: CombatEffect,
}

impl Default for AreaAttackProperty {
    fn default() -> Self {
        Self {
            radius: 50.0,
            cooldown: 2.0,
            effect: CombatEffect::Damage { amount: 4.0 },
        }
    }
}

#[derive(Component, Debug, Clone, Deserialize)]
#[serde(default)]
pub struct BeamAttackProperty {
    /// 宽度
    pub width: f32,
    /// 过热冷却时间
    pub cooldown: f32,
    /// 最大持续时间
    pub duration: f32,
    pub effect: CombatEffect,
}

impl Default for BeamAttackProperty {
    fn default() -> Self {
        Self {
            width: 50.0,
            duration: 10.0,
            cooldown: 2.0,
            effect: CombatEffect::Heal { amount: 4.0 },
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum CombatEffect {
    Damage { amount: f32 },
    Heal { amount: f32 },
    ApplyStatus { status: StatusId },
}

#[derive(Debug, Clone, Deserialize)]
pub enum StatusId {
    DOT,
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
            FixedUpdate,
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

#[cfg(test)]
mod tests {
    use super::{AttackSpec, CombatEffect};

    #[test]
    fn attack_variant_properties_support_empty_ron_defaults() {
        let attack: AttackSpec = ron::from_str("Melee(())").expect("近战默认配置应该可以反序列化");

        let AttackSpec::Melee(property) = attack else {
            panic!("应该反序列化为近战攻击");
        };
        assert_eq!(property.range, 10.0);
        assert_eq!(property.cooldown, 1.0);
        assert!(matches!(
            property.effect,
            CombatEffect::Damage { amount: 1.0 }
        ));
    }
}
