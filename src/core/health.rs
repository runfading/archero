use crate::{GameSet, GameState, RunPhase};
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

    // /// 当前血量百分比
    // pub fn ratio(self) -> f32 {
    //     (self.current / self.max).clamp(0.0, 1.0)
    // }

    /// 扣减血量
    pub fn damage(&mut self, amount: f32) -> f32 {
        let applied = amount.min(self.current);
        self.current -= applied;
        applied
    }

    /// 治疗
    pub fn heal(&mut self, amount: f32) -> f32 {
        self.current = (self.current + amount).max(self.max);
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
    pub source: Entity,
    pub source_name: String,
    pub target: Entity,
    pub target_name: String,
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
            .add_message::<DeathMessage>()
            .add_systems(
                Update,
                apply_health_effects
                    .run_if(in_state(GameState::InGame).and_then(in_state(RunPhase::Playing)))
                    .in_set(GameSet::Core),
            );
    }
}

fn apply_health_effects(
    mut effects: MessageReader<HealthEffectMessage>,
    mut health_query: Query<&mut Health>,
    mut death_writer: MessageWriter<DeathMessage>,
) {
    for message in effects.read() {
        let Ok(mut health) = health_query.get_mut(message.target) else {
            warn!("health target not found");
            continue;
        };

        match message.effect {
            HealthEffect::Damage { amount } => {
                if !amount.is_finite() || amount <= 0.0 {
                    continue;
                }

                let was_alive = health.current > 0.0;
                health.damage(amount);

                info!(
                    "{}对{}造成伤害{}点",
                    message.source_name, message.target_name, health.current
                );
                if was_alive && health.current <= 0.0 {
                    /// 生命值归零发送死亡消息
                    death_writer.write(DeathMessage {
                        entity: message.target,
                        killer: message.source,
                    });
                }
            }

            HealthEffect::Heal { amount } => {
                if !amount.is_finite() || amount <= 0.0 {
                    continue;
                }

                // 这里采用“死亡后不能被普通治疗”的规则。
                if health.current > 0.0 {
                    info!(
                        "{}对{}造成回复{}点",
                        message.source_name, message.target_name, health.current
                    );
                    health.heal(amount);
                }
            }
        }
    }
}
