use crate::asset::GameMeshAssets;
use crate::core::health::{Health, HealthEffect, HealthEffectMessage};
use crate::core::{Faction, RunEntity};
use crate::{GameSet, RunSet};
use avian2d::prelude::{
    Collider, CollisionEventsEnabled, CollisionStart, LinearVelocity, RigidBody, Sensor, SweptCcd,
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// 弹丸属性（挂在武器上）
#[derive(Component, Debug, Clone, Default)]
pub struct ProjectileProperty {
    /// 速度
    pub speed: f32,
    /// 一次发射的投射物数量
    pub num: u32,
    /// 一次冷却开火次数（不是指弹药容量）
    pub fires_num: u32,
    /// 接触伤害
    pub damage: f32,
}

/// 弹丸实体（开火时的弹丸属性）
#[derive(Component, Debug, Clone)]
#[require(Faction, RunEntity)]
pub struct Projectile {
    /// 所有者
    pub owner: Entity,
    /// 方向
    pub dir: Vec2,
    /// 移动速度
    pub speed: f32,
    /// 伤害
    pub damage: f32,
    /// 是否暴击
    pub crit: bool,
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
    mut commands: &mut Commands,
    owner: Entity,
    faction: Faction,
    origin: Vec2,
    dir: Vec2,
    assets: &GameMeshAssets,
) {
    if !origin.is_finite() {
        warn!("拒绝生成位置非法的弹丸: {origin:?}");
        return;
    }

    let Some(dir) = dir.try_normalize() else {
        warn!("拒绝生成方向非法的弹丸: {dir:?}");
        return;
    };

    let angle = dir.to_angle();

    let mat = if faction == Faction::Player {
        assets.mat_arrow.clone()
    } else {
        assets.mat_enemy_shot.clone()
    };

    commands
        .spawn((
            faction,
            Projectile {
                owner,
                dir: dir.normalize(),
                speed: { 360.0 },
                damage: 10.0,
                crit: false,
                pierce: 0,
                ricochet: 0,
                lifetime: 10.0,
                hit: vec![],
            },
            // 碰撞
            RigidBody::Kinematic,
            LinearVelocity(dir.normalize() * 360.0),
            Collider::rectangle(1.0, 1.0),
            Sensor,
            CollisionEventsEnabled,
            SweptCcd::LINEAR,
            // 图形
            Mesh2d(assets.square.clone()),
            MeshMaterial2d(mat),
            Transform::from_translation(origin.extend(0.0))
                .with_rotation(Quat::from_rotation_z(angle))
                .with_scale(Vec3::new(16.0, 4.0, 1.0)),
        ))
        .observe(on_projectile_collision);
}

/// 检查子弹despawn时机
fn update_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Projectile, &mut Transform)>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    for (entity, mut projectile, mut transform) in &mut query {
        let dt = time.delta_secs();
        // 存活时间减少
        projectile.lifetime -= dt;

        let pos = transform.translation.truncate();
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
    projectiles: Query<(&Projectile, &Faction)>,
    targets: Query<(&Faction, Option<&Name>), With<Health>>,
    names: Query<&Name>,
    mut effect_writer: MessageWriter<HealthEffectMessage>,
) {
    // 因为 Observer 挂在弹丸实体上：
    // collider1 一定是被观察的弹丸
    // collider2 是与它碰撞的实体
    let projectile_entity = event.collider1;
    let target_entity = event.collider2;

    let Ok((projectile, projectile_faction)) = projectiles.get(projectile_entity) else {
        return;
    };

    // 墙壁等没有 Health 的实体会在这里被忽略
    let Ok((target_faction, target_name)) = targets.get(target_entity) else {
        return;
    };

    // 过滤友军
    if projectile_faction == target_faction {
        return;
    }

    let source_name = names
        .get(projectile.owner)
        .map(Name::as_str)
        .unwrap_or_default();

    let target_name = target_name.map(Name::as_str).unwrap_or_default();

    effect_writer.write(HealthEffectMessage {
        source: projectile.owner,
        source_name: source_name.to_owned(),
        target: target_entity,
        target_name: target_name.to_owned(),
        effect: HealthEffect::Damage {
            amount: projectile.damage,
        },
    });

    commands.entity(projectile_entity).despawn();
}
