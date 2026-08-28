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
    CriticalChance,
    CriticalMultiplier,
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
    mut commands: Commands,
) {
    let mut commands = &mut commands;

    for message in reader.read() {
        if let Ok(mut item) = query.get_mut(message.effect_entity) {
            let PropertyQueryItem {
                base_damage,
                final_power,
                final_fixed,
                multiplier,
                fixed,
                cooldown,
                critical,
            } = &mut item;

            for (property_type, amount) in message.effect_property.iter() {
                match property_type {
                    PropertyType::BaseDamage => {
                        if let Some(base_damage) = base_damage {
                            base_damage.change(*amount);
                        } else {
                            commands
                                .entity(message.effect_entity)
                                .insert(BaseDamage::new(*amount));
                        }
                    }
                    PropertyType::FinalDamageMultiply => {
                        if let Some(final_power) = final_power {
                            final_power.change(*amount);
                        } else {
                            commands
                                .entity(message.effect_entity)
                                .insert(FinalDamageMultiplyingPower::new(*amount));
                        }
                    }
                    PropertyType::FinalDamageFixed => {
                        if let Some(final_fixed) = final_fixed {
                            final_fixed.change(*amount);
                        } else {
                            commands
                                .entity(message.effect_entity)
                                .insert(FinalFixedDamageBonus::new(*amount));
                        }
                    }
                    PropertyType::DamageMultiply => {
                        if let Some(multiplier) = multiplier {
                            multiplier.change(*amount);
                        } else {
                            commands
                                .entity(message.effect_entity)
                                .insert(DamageMultiplierBonus::new(*amount));
                        }
                    }
                    PropertyType::DamageFixed => {
                        if let Some(fixed_power) = fixed {
                            fixed_power.change(*amount);
                        } else {
                            commands
                                .entity(message.effect_entity)
                                .insert(FixedDamageBonus::new(*amount));
                        }
                    }
                    PropertyType::CooldownTime => {
                        if let Some(cooldown) = cooldown {
                            cooldown.change(*amount);
                        } else {
                            commands
                                .entity(message.effect_entity)
                                .insert(AttackCooldownTime(*amount));
                        }
                    }
                    PropertyType::CriticalChance => {
                        if let Some(critical) = critical {
                            critical.change_chance(*amount);
                        } else {
                            commands
                                .entity(message.effect_entity)
                                .insert(CriticalStats::new_with_chance(*amount));
                        }
                    }
                    PropertyType::CriticalMultiplier => {
                        if let Some(critical) = critical {
                            critical.change_damage_multiplier(*amount);
                        } else {
                            commands
                                .entity(message.effect_entity)
                                .insert(CriticalStats::new_with_multiplier(*amount));
                        }
                    }
                }
            }
        }
    }
}
