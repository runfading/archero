use crate::actors::enemies::EnemyId;
use crate::core::attack::AttackSpec;
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
}

impl Default for EnemyConfig {
    fn default() -> Self {
        Self {
            id: EnemyId::GoblinWarrior,
            move_speed: 60.0,
            base_hp: 60.0,
            spec: AttackSpec::Melee {
                damage: 0.0,
                range: 0.0,
                cooldown: 0.0,
            },
        }
    }
}
