use crate::asset::GameMeshAssets;
use crate::core::health::Health;
use crate::core::health::damage::{DamageMessage, DamageSnapshot};
use crate::core::weapon::FireWeaponMessage;
use crate::core::{Faction, RunEntity};
use crate::{GameSet, RunSet};
use avian2d::prelude::{
    Collider, CollisionEventsEnabled, CollisionStart, LinearVelocity, Position, RigidBody, Sensor,
    SweptCcd,
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
    /// 穿透数量
    pub pierce: u32,
    /// 反弹次数
    pub ricochet: u32,
    /// 存活时间
    pub lifetime: f32,
    /// 已命中实体
    pub hit: Vec<Entity>,
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
    faction: Faction,
    origin: Vec2,
    dir: Vec2,
    speed: f32,
    pierce: u32,
    assets: &GameMeshAssets,
) -> Option<Entity> {
    if !origin.is_finite() {
        warn!("拒绝生成位置非法的弹丸: {origin:?}");
        return None;
    }

    if !speed.is_finite() || speed <= 0.0 {
        warn!("拒绝生成速度非法的弹丸: {speed}");
        return None;
    }

    let Some(dir) = dir.try_normalize() else {
        warn!("拒绝生成方向非法的弹丸: {dir:?}");
        return None;
    };

    let angle = dir.to_angle();

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
                pierce,
                ricochet: 0,
                lifetime: 10.0,
                hit: vec![],
            },
            // 碰撞
            RigidBody::Kinematic,
            LinearVelocity(dir * speed),
            Collider::circle(0.5),
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
    mut query: Query<(Entity, &mut Projectile, &Position)>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    for (entity, mut projectile, position) in &mut query {
        let dt = time.delta_secs();
        // 存活时间减少
        projectile.lifetime -= dt;

        let pos = position.0;
        if projectile.lifetime <= 0.0
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

    if projectile.hit.contains(&target_entity) {
        return;
    }

    effect_writer.write(DamageMessage {
        source: projectile_entity,
        owner: projectile.owner,
        owner_weapon: Some(projectile.owner_weapon),
        target: target_entity,
        snapshot: *snapshot,
    });

    projectile.hit.push(target_entity);
    if projectile.pierce == 0 {
        commands.entity(projectile_entity).despawn();
    } else {
        projectile.pierce -= 1;
    }
}
