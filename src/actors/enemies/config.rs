use crate::actors::enemies::EnemyId;
use crate::core::attack::contact_attack::{ContactDamage, ContactDamageCooldown};
use crate::core::weapon::WeaponId;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EnemyConfig {
    /// 唯一标识
    pub id: EnemyId,
    /// 移动速度
    pub move_speed: f32,
    /// 基础血量
    pub base_hp: f32,
    /// 装备的武器。具体数值从全局武器配置中读取。
    pub weapon: WeaponId,
    /// 接触伤害
    #[serde(default)]
    pub concat_damage: ContactDamage,
    /// 接触伤害触发间隔。
    #[serde(default)]
    pub contact_damage_cooldown: ContactDamageCooldown,
}

#[cfg(test)]
mod tests {
    use super::EnemyConfig;

    #[test]
    fn enemy_weapon_config_uses_default_contact_damage_cooldown() {
        let config: EnemyConfig = ron::from_str(
            r#"(
                id: GoblinWarrior,
                move_speed: 60.0,
                base_hp: 60.0,
                weapon: Bow,
                concat_damage: ContactDamage(1.0),
            )"#,
        )
        .expect("旧敌人配置应保持兼容");

        assert_eq!(config.weapon, crate::core::weapon::WeaponId::Bow);
        assert_eq!(config.contact_damage_cooldown.0, 0.5);
    }
}
