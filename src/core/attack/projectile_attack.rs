use crate::asset::GameMeshAssets;
use crate::core::health::Health;
use crate::core::health::damage::{DamageMessage, DamageSnapshot};
use crate::core::weapon::FireWeaponMessage;
use crate::core::{CollisionLayer, Faction, RunEntity};
use crate::{GameSet, RunSet};
use avian2d::prelude::{
    Collider, CollisionEventsEnabled, CollisionLayers, CollisionStart, LinearVelocity, Position,
    RigidBody, Sensor, SweptCcd,
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// 弹丸实体（开火时的弹丸属性）
#[derive(Component, Debug, Clone)]
#[require(Faction, RunEntity)]
pub struct Projectile {
    /// 所有者
    pub owner: Entity,
    /// 所属武器
    pub owner_weapon: Entity,
    /// 本弹丸还可以命中的目标数量。普通弹丸为 1，每一点穿透额外增加 1。
    pub remaining_hits: u32,
    /// 反弹次数
    pub ricochet: u32,
    /// 存活时间
    pub lifetime: f32,
    /// 剩余可飞行距离。
    pub distance_remaining: f32,
    /// 已命中实体
    pub hit: Vec<Entity>,
    /// 已耗尽命中次数。实体销毁是延迟命令，因此需要立即状态阻止同帧多次命中。
    pub consumed: bool,
}

pub struct ProjectileSpawn {
    pub faction: Faction,
    pub origin: Vec2,
    pub direction: Vec2,
    pub speed: f32,
    pub range: f32,
    pub pierce: u32,
}

impl Projectile {
    fn try_register_hit(&mut self, target: Entity) -> bool {
        if self.consumed || self.hit.contains(&target) {
            return false;
        }

        self.hit.push(target);
        self.remaining_hits = self.remaining_hits.saturating_sub(1);
        self.consumed = self.remaining_hits == 0;
        true
    }
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_lifetime
                .in_set(GameSet::Core)
                .in_set(RunSet::Playing),
        );
    }
}

pub fn spawn_projectile(
    commands: &mut Commands,
    fire: &FireWeaponMessage,
    spawn: ProjectileSpawn,
    assets: &GameMeshAssets,
) -> Option<Entity> {
    let ProjectileSpawn {
        faction,
        origin,
        direction,
        speed,
        range,
        pierce,
    } = spawn;

    if !origin.is_finite() {
        warn!("拒绝生成位置非法的弹丸: {origin:?}");
        return None;
    }

    if !speed.is_finite() || speed <= 0.0 {
        warn!("拒绝生成速度非法的弹丸: {speed}");
        return None;
    }

    if !range.is_finite() || range <= 0.0 {
        warn!("拒绝生成射程非法的弹丸: {range}");
        return None;
    }

    let Some(direction) = direction.try_normalize() else {
        warn!("拒绝生成方向非法的弹丸: {direction:?}");
        return None;
    };

    let angle = direction.to_angle();

    let mat = if faction == Faction::Player {
        assets.mat_arrow.clone()
    } else {
        assets.mat_enemy_shot.clone()
    };

    let projectile = commands
        .spawn((
            faction,
            Projectile {
                owner: fire.owner,
                owner_weapon: fire.weapon,
                remaining_hits: pierce.saturating_add(1),
                ricochet: 0,
                lifetime: 10.0,
                distance_remaining: range,
                hit: vec![],
                consumed: false,
            },
            // 碰撞
            RigidBody::Kinematic,
            LinearVelocity(direction * speed),
            Collider::circle(0.5),
            // 弹丸只检测单位层，双方弹丸之间不会形成碰撞对。
            CollisionLayers::new(CollisionLayer::Projectile, CollisionLayer::Unit),
            Sensor,
            CollisionEventsEnabled,
            SweptCcd::LINEAR,
            // 图形
            Mesh2d(assets.circle.clone()),
            MeshMaterial2d(mat),
            Transform::from_translation(origin.extend(0.0))
                .with_rotation(Quat::from_rotation_z(angle))
                .with_scale(Vec3::new(16.0, 4.0, 1.0)),
        ))
        .observe(on_projectile_collision)
        .id();

    Some(projectile)
}

/// 检查子弹despawn时机
fn update_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Projectile, &Position, &LinearVelocity)>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    for (entity, mut projectile, position, velocity) in &mut query {
        let dt = time.delta_secs();
        // 存活时间减少
        projectile.lifetime -= dt;
        projectile.distance_remaining -= velocity.0.length() * dt;

        let pos = position.0;
        if projectile.consumed
            || projectile.lifetime <= 0.0
            || projectile.distance_remaining <= 0.0
            || pos.x.abs() > window.width() / 2.0 + 40.0
            || pos.y.abs() > window.height() / 2.0 + 40.0
        {
            commands.entity(entity).despawn();
        }
    }
}

/// 子弹碰撞事件
fn on_projectile_collision(
    event: On<CollisionStart>,
    mut commands: Commands,
    mut projectiles: Query<(&mut Projectile, &Faction, &DamageSnapshot)>,
    targets: Query<&Faction, With<Health>>,
    mut effect_writer: MessageWriter<DamageMessage>,
) {
    // 因为 Observer 挂在弹丸实体上：
    // collider1 一定是被观察的弹丸
    // collider2 是与它碰撞的实体
    let projectile_entity = event.collider1;
    let target_entity = event.collider2;

    let Ok((mut projectile, projectile_faction, snapshot)) = projectiles.get_mut(projectile_entity)
    else {
        warn!("弹丸命中时缺少伤害快照: {projectile_entity:?}");
        return;
    };

    // 墙壁等没有 Health 的实体会在这里被忽略
    let Ok(target_faction) = targets.get(target_entity) else {
        return;
    };

    // 过滤友军
    if projectile_faction == target_faction {
        return;
    }

    if !projectile.try_register_hit(target_entity) {
        return;
    }

    effect_writer.write(DamageMessage {
        source: projectile_entity,
        owner: projectile.owner,
        owner_weapon: Some(projectile.owner_weapon),
        target: target_entity,
        snapshot: *snapshot,
    });

    if projectile.consumed {
        commands.entity(projectile_entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use super::Projectile;
    use crate::core::CollisionLayer;
    use avian2d::prelude::CollisionLayers;
    use bevy::prelude::Entity;

    fn projectile(remaining_hits: u32) -> Projectile {
        Projectile {
            owner: Entity::PLACEHOLDER,
            owner_weapon: Entity::PLACEHOLDER,
            remaining_hits,
            ricochet: 0,
            lifetime: 1.0,
            distance_remaining: 100.0,
            hit: Vec::new(),
            consumed: false,
        }
    }

    #[test]
    fn non_piercing_projectile_is_consumed_by_first_unique_target() {
        let mut projectile = projectile(1);
        let first = Entity::from_bits(1);
        let second = Entity::from_bits(2);

        assert!(projectile.try_register_hit(first));
        assert!(projectile.consumed);
        assert!(!projectile.try_register_hit(second));
    }

    #[test]
    fn piercing_projectile_hits_each_target_at_most_once() {
        let mut projectile = projectile(2);
        let first = Entity::from_bits(1);
        let second = Entity::from_bits(2);

        assert!(projectile.try_register_hit(first));
        assert!(!projectile.try_register_hit(first));
        assert!(!projectile.consumed);
        assert!(projectile.try_register_hit(second));
        assert!(projectile.consumed);
    }

    #[test]
    fn projectile_layers_hit_units_but_ignore_other_projectiles() {
        let projectile = CollisionLayers::new(CollisionLayer::Projectile, CollisionLayer::Unit);
        let unit = CollisionLayers::default();

        assert!(projectile.interacts_with(unit));
        assert!(!projectile.interacts_with(projectile));
    }
}
