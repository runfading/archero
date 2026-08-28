use crate::core::health::{HealthEffect, HealthEffectMessage};
use crate::core::property::{CriticalStats, PropertyCalValues, PropertyQuery};
use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use rand::RngExt;

/// 伤害计算开始消息
#[derive(Message, Clone, Copy, Debug)]
pub struct DamageMessage {
    /// 伤害来源
    pub source: Entity,

    /// 所有者
    pub owner: Entity,

    /// 来源武器
    pub owner_weapon: Option<Entity>,

    /// 作用实体
    pub target: Entity,

    /// 命中时携带的伤害快照。
    ///
    /// 即使伤害来源（例如箭矢）在命中后立即销毁，伤害消息仍然可以独立结算。
    pub snapshot: DamageSnapshot,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct DamageSnapshot {
    /// 经过玩家和武器加成后的基础伤害
    pub damage: f32,

    /// 暴击信息
    pub critical: CriticalStats,

    /// 最终伤害倍率
    pub final_damage_multiplying_power: f32,

    /// 最终固定伤害
    pub final_fixed_damage_bonus: f32,
}

impl DamageSnapshot {
    /// 创建不受暴击和额外倍率影响的直接伤害快照。
    pub fn direct(damage: f32) -> Self {
        Self {
            damage,
            critical: CriticalStats::new_with_multiplier(1.0),
            final_damage_multiplying_power: 1.0,
            final_fixed_damage_bonus: 0.0,
        }
    }
}

#[derive(Message, Clone, Copy, Debug)]
pub struct CalDamageSnapshot {
    /// 所有者
    pub owner: Entity,

    /// 来源武器
    pub owner_weapon: Option<Entity>,

    /// 造成伤害的实体
    pub source: Entity,
}

impl CalDamageSnapshot {
    fn snapshot(property: PropertyCalValues) -> DamageSnapshot {
        DamageSnapshot {
            damage: (property.base_damage + property.fixed_damage_bonus)
                * property.damage_multiplier.clamp(0.0, 4.5),
            critical: CriticalStats::builder()
                .chance(property.critical_chance.clamp(0.0, 1.0))
                .damage_multiplier(property.critical_damage_multiplier.clamp(1.0, 3.5)),
            final_damage_multiplying_power: property.final_damage_multiplier.clamp(0.0, 2.5),
            final_fixed_damage_bonus: property.final_fixed_damage_bonus,
        }
    }
}

/// 计算本次伤害快照
pub fn cal_damage_snapshot(
    mut commands: Commands,
    mut cal_damage: MessageReader<CalDamageSnapshot>,
    query: Query<PropertyQuery>,
) {
    for message in cal_damage.read() {
        let Ok(stats) = query.get(message.owner) else {
            warn!("伤害所有者不存在: {:?}", message.owner);
            continue;
        };

        let mut values = stats.values();
        if let Some(weapon) = message.owner_weapon {
            let Ok(weapon_stats) = query.get(weapon) else {
                warn!("伤害武器不存在: {weapon:?}");
                continue;
            };
            values = values.merge(weapon_stats.values());
        }

        commands
            .entity(message.source)
            .insert(CalDamageSnapshot::snapshot(values));
    }
}

/// 伤害应用
pub fn apply_damage(
    mut damage_start: MessageReader<DamageMessage>,
    mut health_effect_writer: MessageWriter<HealthEffectMessage>,
) {
    for message in damage_start.read() {
        let snapshot = message.snapshot;
        let damage = if rand::rng().random::<f32>() < snapshot.critical.chance {
            snapshot.damage
                * snapshot.critical.damage_multiplier
                * snapshot.final_damage_multiplying_power
                + snapshot.final_fixed_damage_bonus
        } else {
            snapshot.damage * snapshot.final_damage_multiplying_power
                + snapshot.final_fixed_damage_bonus
        };

        health_effect_writer.write(HealthEffectMessage {
            damage: *message,
            effect: HealthEffect::Damage { amount: damage },
        });
    }
}
