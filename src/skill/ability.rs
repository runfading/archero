use crate::core::ability::{
    AbilityQuery, AbilityQueryItem, Ejection, Forward, Multiple, Oblique, Pierce,
};
use crate::skill::SkillSet;
use bevy::app::{App, Plugin};
use bevy::prelude::*;

#[derive(Message, Debug, Clone)]
pub struct AbilityChangeMessage {
    /// 来源
    pub source: Entity,
    /// 作用对象
    pub effect_entity: Entity,
    /// 作用数值
    pub effect_property: Vec<(AbilityType, u32)>,
}

#[derive(PartialEq, Eq, Clone, Hash, Copy, Debug)]
pub enum AbilityType {
    /// 正向箭
    Forward,
    /// 斜向箭
    Oblique,
    /// 多重箭
    Multiple,
    /// 弹射箭
    Ejection,
    /// 穿透
    Pierce,
}

/// 属性类
pub struct SkillAbilityPlugin;

impl Plugin for SkillAbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<AbilityChangeMessage>()
            .add_systems(FixedUpdate, deal_ability_change.in_set(SkillSet::Effective));
    }
}

fn deal_ability_change(
    mut commands: Commands,
    mut reader: PopulatedMessageReader<AbilityChangeMessage>,
    mut query: Query<AbilityQuery>,
) {
    let mut commands = &mut commands;

    for message in reader.read() {
        if let Ok(mut item) = query.get_mut(message.effect_entity) {
            let AbilityQueryItem {
                forward,
                oblique,
                multiple,
                pierce,
                ejection,
            } = &mut item;

            for (ability_type, amount) in message.effect_property.iter() {
                match ability_type {
                    AbilityType::Forward => {
                        if let Some(forwar) = forward {
                            forwar.change(*amount);
                        } else {
                            commands
                                .entity(message.effect_entity)
                                .insert(Forward(*amount));
                        }
                    }
                    AbilityType::Oblique => {
                        if let Some(oblique) = oblique {
                            oblique.change(*amount);
                        } else {
                            commands
                                .entity(message.effect_entity)
                                .insert(Oblique(*amount));
                        }
                    }
                    AbilityType::Multiple => {
                        if let Some(multiple) = multiple {
                            multiple.change(*amount);
                        } else {
                            commands
                                .entity(message.effect_entity)
                                .insert(Multiple(*amount));
                        }
                    }
                    AbilityType::Ejection => {
                        if let Some(ejection) = ejection {
                            ejection.change(*amount);
                        } else {
                            commands
                                .entity(message.effect_entity)
                                .insert(Ejection(*amount));
                        }
                    }
                    AbilityType::Pierce => {
                        if let Some(pierce) = pierce {
                            pierce.change(*amount);
                        } else {
                            commands
                                .entity(message.effect_entity)
                                .insert(Pierce(*amount));
                        }
                    }
                }
            }
        }
    }
}
