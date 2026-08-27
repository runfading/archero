use crate::core::health::{HealthEffect, HealthEffectMessage};
use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use rand::RngExt;

/// 基础伤害
///
/// 武器或玩家的基础伤害
///
/// 生成后永远不会发生变化
#[derive(Component, Clone, Copy)]
pub struct BaseDamage(f32);

impl Default for BaseDamage {
    fn default() -> Self {
        BaseDamage(0.0)
    }
}

impl BaseDamage {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn get(&self) -> f32 {
        self.0
    }
}
/// 最终伤害倍率（负数减伤，正数加伤）
///
/// 对所有增伤应用完成后的数值进行增减
#[derive(Component, Clone, Copy)]
pub struct FinalDamageMultiplyingPower(pub f32);

impl Default for FinalDamageMultiplyingPower {
    fn default() -> Self {
        FinalDamageMultiplyingPower(1.0)
    }
}

/// 最终固定伤害（负数减伤，正数加伤）
///
/// 对所有增伤应用完成后的数值进行增减
#[derive(Component, Clone, Copy)]
pub struct FinalFixedDamageBonus(pub f32);

impl Default for FinalFixedDamageBonus {
    fn default() -> Self {
        FinalFixedDamageBonus(0.0)
    }
}

/// 固定伤害加持（负数减伤，正数加伤）
#[derive(Component, Clone, Copy)]
pub struct FixedDamageBonus(pub f32);

impl Default for FixedDamageBonus {
    fn default() -> Self {
        FixedDamageBonus(0.0)
    }
}

/// 伤害倍率加成 （负数减伤，正数加伤）
///
/// 针对基础伤害的加持，不对增伤后的数据调整
#[derive(Component, Clone, Copy)]
pub struct DamageMultiplierBonus(f32);

impl Default for DamageMultiplierBonus {
    fn default() -> Self {
        DamageMultiplierBonus(1.)
    }
}

impl DamageMultiplierBonus {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn get(&self) -> f32 {
        self.0
    }
}

/// 暴击配置
#[derive(Component, Clone, Copy, Debug)]
pub struct CriticalStats {
    /// 暴击概率，0.0～1.0
    pub chance: f32,

    /// 暴击后的总伤害倍率，例如 1.5 表示造成 150% 伤害
    pub damage_multiplier: f32,
}

impl Default for CriticalStats {
    fn default() -> Self {
        Self {
            chance: 0.05,
            damage_multiplier: 1.5,
        }
    }
}

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
            critical: CriticalStats {
                chance: 0.0,
                damage_multiplier: 1.0,
            },
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

#[derive(QueryData)]
pub struct DamageCalcQuery {
    pub base_damage: Option<&'static BaseDamage>,
    pub final_power: Option<&'static FinalDamageMultiplyingPower>,
    pub final_fixed: Option<&'static FinalFixedDamageBonus>,
    pub multiplier: Option<&'static DamageMultiplierBonus>,
    pub fixed: Option<&'static FixedDamageBonus>,
    pub critical: Option<&'static CriticalStats>,
}

#[derive(Clone, Copy, Debug)]
struct DamageCalcValues {
    base_damage: f32,
    fixed_damage_bonus: f32,
    damage_multiplier: f32,
    final_damage_multiplier: f32,
    final_fixed_damage_bonus: f32,
    critical_chance: f32,
    critical_damage_multiplier: f32,
}

impl Default for DamageCalcValues {
    fn default() -> Self {
        Self {
            base_damage: 0.0,
            fixed_damage_bonus: 0.0,
            damage_multiplier: 1.0,
            final_damage_multiplier: 1.0,
            final_fixed_damage_bonus: 0.0,
            critical_chance: 0.0,
            critical_damage_multiplier: 1.0,
        }
    }
}

impl<'w, 's> DamageCalcQueryItem<'w, 's> {
    fn values(&self) -> DamageCalcValues {
        let critical = self.critical.copied().unwrap_or(CriticalStats {
            chance: 0.0,
            damage_multiplier: 1.0,
        });

        DamageCalcValues {
            base_damage: self.base_damage.map_or(0.0, BaseDamage::get),
            fixed_damage_bonus: self.fixed.map_or(0.0, |value| value.0),
            damage_multiplier: self.multiplier.map_or(1.0, DamageMultiplierBonus::get),
            final_damage_multiplier: self.final_power.map_or(1.0, |value| value.0),
            final_fixed_damage_bonus: self.final_fixed.map_or(0.0, |value| value.0),
            critical_chance: critical.chance,
            critical_damage_multiplier: critical.damage_multiplier,
        }
    }
}

impl DamageCalcValues {
    fn merge(self, other: Self) -> Self {
        Self {
            base_damage: self.base_damage + other.base_damage,
            fixed_damage_bonus: self.fixed_damage_bonus + other.fixed_damage_bonus,
            damage_multiplier: self.damage_multiplier * other.damage_multiplier,
            final_damage_multiplier: self.final_damage_multiplier * other.final_damage_multiplier,
            final_fixed_damage_bonus: self.final_fixed_damage_bonus
                + other.final_fixed_damage_bonus,
            critical_chance: (self.critical_chance + other.critical_chance).clamp(0.0, 1.0),
            // 每个来源保存的是“总暴击倍率”，合并时只叠加超出 1.0 的部分。
            critical_damage_multiplier: (1.0
                + (self.critical_damage_multiplier - 1.0)
                + (other.critical_damage_multiplier - 1.0))
                .clamp(1.0, 3.5),
        }
    }

    fn snapshot(self) -> DamageSnapshot {
        DamageSnapshot {
            damage: (self.base_damage + self.fixed_damage_bonus)
                * self.damage_multiplier.clamp(0.0, 4.5),
            critical: CriticalStats {
                chance: self.critical_chance.clamp(0.0, 1.0),
                damage_multiplier: self.critical_damage_multiplier.clamp(1.0, 3.5),
            },
            final_damage_multiplying_power: self.final_damage_multiplier.clamp(0.0, 2.5),
            final_fixed_damage_bonus: self.final_fixed_damage_bonus,
        }
    }
}

/// 计算本次伤害快照
pub fn cal_damage_snapshot(
    mut commands: Commands,
    mut cal_damage: MessageReader<CalDamageSnapshot>,
    query: Query<DamageCalcQuery>,
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

        commands.entity(message.source).insert(values.snapshot());
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

#[cfg(test)]
mod tests {
    use super::DamageCalcValues;

    #[test]
    fn damage_pipeline_uses_additive_flat_and_multiplicative_factor_stages() {
        let snapshot = DamageCalcValues {
            base_damage: 10.0,
            fixed_damage_bonus: 2.0,
            damage_multiplier: 1.5,
            final_damage_multiplier: 0.8,
            final_fixed_damage_bonus: 3.0,
            critical_chance: 0.25,
            critical_damage_multiplier: 2.0,
        }
        .snapshot();

        assert_eq!(snapshot.damage, 18.0);
        assert_eq!(snapshot.final_damage_multiplying_power, 0.8);
        assert_eq!(snapshot.final_fixed_damage_bonus, 3.0);
        assert_eq!(snapshot.critical.chance, 0.25);
        assert_eq!(snapshot.critical.damage_multiplier, 2.0);
    }

    #[test]
    fn owner_and_weapon_multipliers_are_composed_multiplicatively() {
        let owner = DamageCalcValues {
            base_damage: 2.0,
            damage_multiplier: 1.2,
            critical_damage_multiplier: 1.5,
            ..Default::default()
        };
        let weapon = DamageCalcValues {
            base_damage: 8.0,
            damage_multiplier: 1.5,
            critical_damage_multiplier: 1.5,
            ..Default::default()
        };

        let snapshot = owner.merge(weapon).snapshot();

        assert!((snapshot.damage - 18.0).abs() < f32::EPSILON);
        assert_eq!(snapshot.critical.damage_multiplier, 2.0);
    }
}
