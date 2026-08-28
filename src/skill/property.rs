use crate::actors::player::Player;
use crate::core::health::Health;
use crate::core::property::*;
use crate::skill::SkillSet;
use bevy::prelude::*;

#[derive(Message, Debug, Clone)]
pub struct PropertyChangeMessage {
    /// 来源
    pub source: Entity,
    /// 作用对象
    pub effect_entity: Entity,
    /// 作用数值
    pub effect_property: Vec<(PropertyType, f32)>,
}

#[derive(PartialEq, Eq, Clone, Hash, Copy, Debug)]
pub enum PropertyType {
    BaseDamage,
    FinalDamageMultiply,
    FinalDamageFixed,
    DamageMultiply,
    DamageFixed,
    CooldownTime,
    /// 按当前攻击间隔的比例调整，例如 -0.12 表示减少 12%。
    CooldownRatio,
    CriticalChance,
    CriticalMultiplier,
    /// 按当前最大生命的比例增加上限，并回复相同的增加值。
    MaxHealthRatio,
    /// 按当前移动速度的比例调整。
    MoveSpeedRatio,
}

/// 属性类
pub struct SkillPropertyPlugin;

impl Plugin for SkillPropertyPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PropertyChangeMessage>().add_systems(
            FixedUpdate,
            deal_property_change.in_set(SkillSet::Effective),
        );
    }
}

fn deal_property_change(
    mut reader: PopulatedMessageReader<PropertyChangeMessage>,
    mut query: Query<PropertyQuery>,
    mut health_query: Query<&mut Health>,
    mut player_query: Query<&mut Player>,
    mut commands: Commands,
) {
    for message in reader.read() {
        if let Ok(mut item) = query.get_mut(message.effect_entity) {
            for (property_type, amount) in message.effect_property.iter() {
                match property_type {
                    PropertyType::BaseDamage => {
                        if let Some(base_damage) = &mut item.base_damage {
                            base_damage.change(*amount);
                        } else {
                            commands
                                .entity(message.effect_entity)
                                .insert(BaseDamage::new(*amount));
                        }
                    }
                    PropertyType::FinalDamageMultiply => {
                        if let Some(final_power) = &mut item.final_power {
                            final_power.change(*amount);
                        } else {
                            commands
                                .entity(message.effect_entity)
                                .insert(FinalDamageMultiplyingPower::new(1.0 + *amount));
                        }
                    }
                    PropertyType::FinalDamageFixed => {
                        if let Some(final_fixed) = &mut item.final_fixed {
                            final_fixed.change(*amount);
                        } else {
                            commands
                                .entity(message.effect_entity)
                                .insert(FinalFixedDamageBonus::new(*amount));
                        }
                    }
                    PropertyType::DamageMultiply => {
                        if let Some(multiplier) = &mut item.multiplier {
                            multiplier.change(*amount);
                        } else {
                            commands
                                .entity(message.effect_entity)
                                .insert(DamageMultiplierBonus::new(1.0 + *amount));
                        }
                    }
                    PropertyType::DamageFixed => {
                        if let Some(fixed_power) = &mut item.fixed {
                            fixed_power.change(*amount);
                        } else {
                            commands
                                .entity(message.effect_entity)
                                .insert(FixedDamageBonus::new(*amount));
                        }
                    }
                    PropertyType::CooldownTime => {
                        if let Some(cooldown) = &mut item.cooldown {
                            cooldown.change(*amount);
                        } else {
                            let mut cooldown = AttackCooldownTime::default();
                            cooldown.change(*amount);
                            commands.entity(message.effect_entity).insert(cooldown);
                        }
                    }
                    PropertyType::CooldownRatio => {
                        if let Some(cooldown) = &mut item.cooldown {
                            cooldown.change_ratio(*amount);
                        } else {
                            warn!(
                                "无法按比例调整缺少 AttackCooldownTime 的实体 {:?}",
                                message.effect_entity
                            );
                        }
                    }
                    PropertyType::CriticalChance => {
                        if let Some(critical) = &mut item.critical {
                            critical.change_chance(*amount);
                        } else {
                            commands
                                .entity(message.effect_entity)
                                .insert(CriticalStats::new_with_chance(*amount));
                        }
                    }
                    PropertyType::CriticalMultiplier => {
                        if let Some(critical) = &mut item.critical {
                            critical.change_damage_multiplier(*amount);
                        } else {
                            commands
                                .entity(message.effect_entity)
                                .insert(CriticalStats::new_with_multiplier(1.0 + *amount));
                        }
                    }
                    PropertyType::MaxHealthRatio => {
                        let Ok(mut health) = health_query.get_mut(message.effect_entity) else {
                            warn!("无法调整缺少 Health 的实体 {:?}", message.effect_entity);
                            continue;
                        };
                        let old_max = health.max.max(1.0);
                        let new_max = (old_max * (1.0 + *amount)).max(1.0);
                        let increase = (new_max - old_max).max(0.0);
                        health.max = new_max;
                        health.current = (health.current + increase).clamp(0.0, new_max);
                    }
                    PropertyType::MoveSpeedRatio => {
                        let Ok(mut player) = player_query.get_mut(message.effect_entity) else {
                            warn!("无法调整缺少 Player 的实体 {:?}", message.effect_entity);
                            continue;
                        };
                        player.move_speed = (player.move_speed * (1.0 + *amount)).max(0.0);
                    }
                }
            }
        }
    }
}
