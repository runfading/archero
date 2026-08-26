use crate::core::Faction;
use crate::core::health::{HealthEffect, HealthEffectMessage};
use crate::{GameSet, GameState, RunPhase, RunSet};
use avian2d::prelude::CollisionStart;
use bevy::prelude::*;

pub struct CollisionAttackPlugin;
impl Plugin for CollisionAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            collision_attack
                .in_set(GameSet::Core)
                .in_set(RunSet::Playing),
        );
    }
}

fn collision_attack(
    mut collisions: MessageReader<CollisionStart>,
    factions: Query<&Faction>,
    name: Query<&Name>,
    mut message_writer: MessageWriter<HealthEffectMessage>,
) {
    for collision in collisions.read() {
        let Ok(faction1) = factions.get(collision.collider1) else {
            warn!("collider1 not found");
            continue;
        };

        let Ok(faction2) = factions.get(collision.collider2) else {
            warn!("collider2 not found");
            continue;
        };

        let Ok(name1) = name.get(collision.collider1) else {
            warn!("collider1 name not found");
            continue;
        };

        let Ok(name2) = name.get(collision.collider2) else {
            warn!("collider2 name not found");
            continue;
        };

        // 不攻击同阵营单位。
        if faction1 == faction2 {
            continue;
        }

        /// 碰撞时，现在设定成都收到伤害
        message_writer.write(HealthEffectMessage {
            source: collision.collider1,
            source_name: name1.to_string(),
            target: collision.collider2,
            target_name: name2.to_string(),
            effect: HealthEffect::Damage { amount: 10. },
        });

        message_writer.write(HealthEffectMessage {
            source: collision.collider2,
            source_name: name2.to_string(),
            target: collision.collider1,
            target_name: name1.to_string(),
            effect: HealthEffect::Damage { amount: 10. },
        });
    }
}
