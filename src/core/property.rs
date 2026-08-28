use bevy::ecs::query::QueryData;
use bevy::prelude::Component;

/// 基础伤害
///
/// 武器或玩家的基础伤害
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

    pub fn change(&mut self, amount: f32) {
        self.0 += amount;
    }
}
/// 最终伤害倍率（负数减伤，正数加伤）
///
/// 对所有增伤应用完成后的数值进行增减
#[derive(Component, Clone, Copy)]
pub struct FinalDamageMultiplyingPower(f32);

impl Default for FinalDamageMultiplyingPower {
    fn default() -> Self {
        FinalDamageMultiplyingPower(1.0)
    }
}

impl FinalDamageMultiplyingPower {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn change(&mut self, amount: f32) {
        self.0 += amount;
    }
}

/// 最终固定伤害（负数减伤，正数加伤）
///
/// 对所有增伤应用完成后的数值进行增减
#[derive(Component, Clone, Copy)]
pub struct FinalFixedDamageBonus(f32);

impl Default for FinalFixedDamageBonus {
    fn default() -> Self {
        FinalFixedDamageBonus(0.0)
    }
}

impl FinalFixedDamageBonus {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn change(&mut self, amount: f32) {
        self.0 += amount;
    }
}

/// 固定伤害加持（负数减伤，正数加伤）
#[derive(Component, Clone, Copy)]
pub struct FixedDamageBonus(f32);

impl Default for FixedDamageBonus {
    fn default() -> Self {
        FixedDamageBonus(0.0)
    }
}

impl FixedDamageBonus {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn change(&mut self, amount: f32) {
        self.0 += amount;
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

    pub fn change(&mut self, amount: f32) {
        self.0 += amount;
    }
}

#[derive(Component, Clone, Copy)]
pub struct AttackCooldownTime(pub f32);

impl Default for AttackCooldownTime {
    fn default() -> Self {
        Self(1.0)
    }
}

impl AttackCooldownTime {
    pub fn change(&mut self, amount: f32) {
        // 最低 0.05 秒，避免零冷却造成每帧无限攻击。
        self.0 = (self.0 + amount).max(0.05);
    }

    pub fn change_ratio(&mut self, ratio: f32) {
        self.0 = (self.0 * (1.0 + ratio)).max(0.05);
    }
}

/// 暴击配置
#[derive(Component, Clone, Copy, Debug)]
pub struct CriticalStats {
    /// 暴击概率，0.0～1.0
    pub(crate) chance: f32,

    /// 暴击后的总伤害倍率，例如 1.5 表示造成 150% 伤害
    pub(crate) damage_multiplier: f32,
}

impl Default for CriticalStats {
    fn default() -> Self {
        Self {
            chance: 0.00,
            damage_multiplier: 1.0,
        }
    }
}

impl CriticalStats {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn chance(self, value: f32) -> Self {
        Self {
            chance: value,
            ..self
        }
    }

    pub fn damage_multiplier(self, value: f32) -> Self {
        Self {
            damage_multiplier: value,
            ..self
        }
    }

    pub fn new_with_chance(value: f32) -> Self {
        Self {
            chance: value,
            damage_multiplier: 1.0,
        }
    }

    pub fn new_with_multiplier(value: f32) -> Self {
        Self {
            chance: 0.0,
            damage_multiplier: value,
        }
    }

    pub fn change_chance(&mut self, amount: f32) {
        self.chance = (self.chance + amount).clamp(0.0, 1.0);
    }

    pub fn change_damage_multiplier(&mut self, amount: f32) {
        self.damage_multiplier = (self.damage_multiplier + amount).clamp(1.0, 3.5);
    }
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct PropertyQuery<'a> {
    pub base_damage: Option<&'a mut BaseDamage>,
    pub final_power: Option<&'a mut FinalDamageMultiplyingPower>,
    pub final_fixed: Option<&'a mut FinalFixedDamageBonus>,
    pub multiplier: Option<&'a mut DamageMultiplierBonus>,
    pub fixed: Option<&'a mut FixedDamageBonus>,
    pub cooldown: Option<&'a mut AttackCooldownTime>,
    pub critical: Option<&'a mut CriticalStats>,
}

#[derive(Clone, Copy, Debug)]
pub struct PropertyCalValues {
    pub base_damage: f32,
    pub fixed_damage_bonus: f32,
    pub damage_multiplier: f32,
    pub final_damage_multiplier: f32,
    pub final_fixed_damage_bonus: f32,
    pub critical_chance: f32,
    pub critical_damage_multiplier: f32,
}

impl Default for PropertyCalValues {
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

impl<'w, 's, 'a> PropertyQueryReadOnlyItem<'w, 's, 'a> {
    pub fn values(&self) -> PropertyCalValues {
        let critical = self.critical.copied().unwrap_or(CriticalStats {
            chance: 0.0,
            damage_multiplier: 1.0,
        });

        PropertyCalValues {
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

impl PropertyCalValues {
    pub fn merge(self, other: Self) -> Self {
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
}
