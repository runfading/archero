use crate::core::RunEntity;
use bevy::prelude::Component;
use serde::Deserialize;

#[derive(Component, Debug, Clone, Deserialize)]
#[require(RunEntity)]
pub enum AttackSpec {
    /// 近战
    Melee {
        #[serde(default = "AttackSpec::default_melee_damage")]
        damage: f32,
        #[serde(default = "AttackSpec::default_melee_range")]
        range: f32,
        #[serde(default = "AttackSpec::default_melee_cooldown")]
        cooldown: f32,
    },
    /// 投掷物/发射物
    Projectile {
        #[serde(default = "AttackSpec::default_projectile_damage")]
        damage: f32,
        #[serde(default = "AttackSpec::default_projectile_range")]
        range: f32,
        #[serde(default = "AttackSpec::default_projectile_cooldown")]
        cooldown: f32,
        #[serde(default = "AttackSpec::default_projectile_speed")]
        projectile_speed: f32,
    },
    /// 法术/技能
    Spell {
        #[serde(default = "AttackSpec::default_spell_damage")]
        damage: f32,
        #[serde(default = "AttackSpec::default_spell_radius")]
        radius: f32,
        #[serde(default = "AttackSpec::default_spell_cooldown")]
        cooldown: f32,
    },
    /// 治疗
    Heal {
        #[serde(default = "AttackSpec::default_heal_amount")]
        amount: f32,
        #[serde(default = "AttackSpec::default_heal_radius")]
        radius: f32,
        #[serde(default = "AttackSpec::default_heal_cooldown")]
        cooldown: f32,
    },
}

/// default config
impl AttackSpec {
    fn default_melee_damage() -> f32 {
        1.0
    }

    fn default_melee_range() -> f32 {
        10.0
    }

    fn default_melee_cooldown() -> f32 {
        1.0
    }

    fn default_projectile_damage() -> f32 {
        2.0
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

    fn default_spell_damage() -> f32 {
        4.0
    }

    fn default_spell_radius() -> f32 {
        50.0
    }

    fn default_spell_cooldown() -> f32 {
        2.0
    }

    fn default_heal_amount() -> f32 {
        4.0
    }

    fn default_heal_radius() -> f32 {
        50.0
    }

    fn default_heal_cooldown() -> f32 {
        2.0
    }
}
