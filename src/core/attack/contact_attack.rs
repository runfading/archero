use crate::core::Faction;
use crate::core::attack::Knockback;
use crate::core::health::Health;
use crate::core::health::damage::{DamageMessage, DamageSnapshot};
use avian2d::prelude::{CollisionStart, LinearVelocity, Position};
use bevy::prelude::*;
use serde::Deserialize;

/// 应用碰撞伤害
#[derive(Component, Deserialize, Default, Debug, Clone)]
#[require(ContactKnockback)]
pub struct ContactDamage(pub f32);

/// 接触伤害命中敌对单位时的弹开参数。
///
/// `speed` 是双方弹开的速度，`duration` 是移动控制暂停覆盖速度的时间。
#[derive(Component, Debug, Clone)]
pub struct ContactKnockback {
    pub speed: f32,
    pub duration: f32,
}

impl Default for ContactKnockback {
    fn default() -> Self {
        Self {
            speed: 220.0,
            duration: 0.12,
        }
    }
}

/// 带有 [`ContactDamage`] 的实体开始接触其他单位时造成伤害。
///
/// 该 Observer 应绑定在同时具有 `CollisionEventsEnabled` 的攻击者实体上。
pub fn on_contact_damage(
    event: On<CollisionStart>,
    mut commands: Commands,
    attackers: Query<(&Faction, &ContactDamage, &ContactKnockback)>,
    targets: Query<&Faction, With<Health>>,
    positions: Query<&Position>,
    mut velocities: Query<&mut LinearVelocity>,
    mut message_writer: MessageWriter<DamageMessage>,
) {
    // 对实体级 Observer，Avian 保证 collider1 是 Observer 所属实体。
    let source = event.collider1;
    let target = event.collider2;

    let Ok((source_faction, damage, knockback)) = attackers.get(source) else {
        return;
    };
    let Ok(target_faction) = targets.get(target) else {
        return;
    };

    if source_faction == target_faction || !damage.0.is_finite() || damage.0 <= 0.0 {
        return;
    }

    message_writer.write(DamageMessage {
        source,
        owner: source,
        owner_weapon: None,
        target,
        snapshot: DamageSnapshot::direct(damage.0),
    });

    if !knockback.speed.is_finite()
        || knockback.speed <= 0.0
        || !knockback.duration.is_finite()
        || knockback.duration <= 0.0
    {
        return;
    }

    let Ok(source_position) = positions.get(source) else {
        return;
    };
    let Ok(target_position) = positions.get(target) else {
        return;
    };

    let separation = target_position.0 - source_position.0;
    let direction = if separation.length_squared() > f32::EPSILON {
        separation.normalize()
    } else {
        Vec2::X
    };

    let Ok([mut source_velocity, mut target_velocity]) = velocities.get_many_mut([source, target])
    else {
        return;
    };

    source_velocity.0 = -direction * knockback.speed;
    target_velocity.0 = direction * knockback.speed;
    commands
        .entity(source)
        .insert(Knockback::new(knockback.duration));
    commands
        .entity(target)
        .insert(Knockback::new(knockback.duration));
}
