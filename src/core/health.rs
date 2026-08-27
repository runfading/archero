pub mod damage;
pub mod heal;

use crate::core::health::damage::{
    CalDamageSnapshot, DamageMessage, apply_damage, cal_damage_snapshot,
};
use crate::core::weapon::WeaponSet;
use crate::{GameSet, RunSet};
use bevy::prelude::*;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    /// 满生命值
    pub fn full(max: f32) -> Self {
        Self { current: max, max }
    }

    /// 当前血量百分比。
    pub fn ratio(self) -> f32 {
        if !self.current.is_finite() || !self.max.is_finite() || self.max <= 0.0 {
            return 0.0;
        }

        (self.current / self.max).clamp(0.0, 1.0)
    }

    /// 扣减血量
    pub fn damage(&mut self, amount: f32) -> f32 {
        let applied = amount.min(self.current);
        self.current -= applied;
        applied
    }

    /// 治疗
    pub fn heal(&mut self, amount: f32) -> f32 {
        self.current = (self.current + amount).min(self.max);
        self.current
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HealthEffect {
    Damage { amount: f32 },
    Heal { amount: f32 },
}

/// 生命值变动消息
#[derive(Message, Debug, Clone)]
pub struct HealthEffectMessage {
    pub damage: DamageMessage,
    pub effect: HealthEffect,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct DeathMessage {
    pub entity: Entity,
    pub killer: Entity,
}

pub struct HealthPlugin;
impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<HealthEffectMessage>()
            .add_message::<DamageMessage>()
            .add_message::<CalDamageSnapshot>()
            .add_message::<DeathMessage>()
            .add_systems(
                Update,
                (
                    cal_damage_snapshot.after(WeaponSet::Attack),
                    apply_damage,
                    apply_health_effects,
                )
                    .chain()
                    .in_set(GameSet::Core)
                    .in_set(RunSet::Playing),
            );
    }
}

fn apply_health_effects(
    mut effects: MessageReader<HealthEffectMessage>,
    mut health_query: Query<(&mut Health, Option<&Name>)>,
    name_query: Query<&Name>,
    mut death_writer: MessageWriter<DeathMessage>,
) {
    for message in effects.read() {
        let target = message.damage.target;
        let source = message.damage.owner;

        let (mut health, target_name) = match health_query.get_mut(target) {
            Ok((health, name)) => (health, name.map_or_default(|name| name.as_str())),
            Err(_) => {
                warn!("health target not found");
                continue;
            }
        };
        let source_name = name_query.get(source).map_or_default(|t| t.as_str());

        match message.effect {
            HealthEffect::Damage { amount } => {
                if !amount.is_finite() || amount <= 0.0 {
                    continue;
                }

                let was_alive = health.current > 0.0;
                health.damage(amount);

                info!(
                    "{}对{}造成伤害{}点,剩余{}",
                    source_name, target_name, amount, health.current
                );
                if was_alive && health.current <= 0.0 {
                    // 生命值归零发送死亡消息
                    death_writer.write(DeathMessage {
                        entity: target,
                        killer: source,
                    });
                }
            }

            HealthEffect::Heal { amount } => {
                if !amount.is_finite() || amount <= 0.0 {
                    continue;
                }

                // 这里采用“死亡后不能被普通治疗”的规则。
                if health.current > 0.0 {
                    health.heal(amount);
                    info!(
                        "{}对{}回复{}点,当前{}",
                        source_name, target_name, amount, health.current
                    );
                }
            }
        }
    }
}
