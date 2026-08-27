use crate::actors::enemies::EnemyId;
use crate::core::attack::AttackSpec;
use crate::core::attack::contact_attack::ContactDamage;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EnemyConfig {
    /// 唯一标识
    pub id: EnemyId,
    /// 移动速度
    pub move_speed: f32,
    /// 基础血量
    pub base_hp: f32,
    /// 攻击配置
    pub spec: AttackSpec,
    /// 接触伤害
    #[serde(default)]
    pub concat_damage: ContactDamage,
}
