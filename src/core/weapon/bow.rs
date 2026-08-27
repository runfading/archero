use crate::actors::enemies::Enemy;
use crate::asset::GameMeshAssets;
use crate::core::Faction;
use crate::core::attack::projectile_attack::{ProjectileProperty, spawn_projectile};
use crate::core::attack::{AttackSpec, AttackTriggerRange, CombatEffect};
use crate::core::weapon::config::WeaponConfig;
use crate::core::weapon::{Cooldown, FireWeaponMessage, WeaponId, WeaponSet};
use crate::{GameSet, RunSet};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::scene::{bsn, template_value};

pub struct BowPlugin;
impl Plugin for BowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            request_weapon_fire
                .in_set(GameSet::Core)
                .in_set(RunSet::Playing)
                .in_set(WeaponSet::RequestFire),
        );
    }
}

pub fn spawn_bow(mut commands: &mut Commands, owner: Entity, config: &WeaponConfig) {
    let (range, cooldown, projectile_speed, amount) = match &config.attack {
        AttackSpec::Projectile {
            range,
            cooldown,
            projectile_speed,
            effect,
        } => {
            let amount = match effect {
                CombatEffect::Damage { amount } => amount,
                CombatEffect::Heal { .. } => {
                    warn!("弓箭攻击方式不支持治疗");
                    return;
                }
            };

            (range, cooldown, projectile_speed, amount)
        }
        _ => {
            warn!("弓箭攻击方式暂时只支持弹道，不支持{:?}", config.attack);
            return;
        }
    };
    let weapon = commands
        .spawn_scene(bsn! {
            #bow
            WeaponId::Bow
            template_value(config.targeting.clone())
            template_value(Cooldown{ timer: Timer::from_seconds(*cooldown,TimerMode::Repeating) })
            template_value(ProjectileProperty {
                speed: *projectile_speed,
                num: 1,
                fires_num: 1,
                damage: *amount,
            })
            template_value(AttackTriggerRange { range: *range })
        })
        .insert(Transform::default())
        .id();

    commands.entity(owner).add_child(weapon);
}

/// weapon 是否冷却好了
fn request_weapon_fire(
    time: Res<Time>,
    mut cooldown_query: Query<(
        // 武器entity
        Entity,
        &ChildOf,
        // 武器是玩家的 child，因此这里也应使用世界坐标：不然得到的是武器相对玩家的局部坐标
        &GlobalTransform,
        &AttackTriggerRange,
        &mut Cooldown,
        &ProjectileProperty,
    )>,
    spatial_query: SpatialQuery,
    // 先过滤掉敌人，后面再想办法
    colliders: Query<(&Collider, &Position, &Rotation), With<Enemy>>,
    mut writer: MessageWriter<FireWeaponMessage>,
) {
    for (weapon, child_of, transform, trigger_range, mut cooldown, _projectile) in
        &mut cooldown_query
    {
        // 当前查询匹配到的子实体，也就是武器

        // weapon 的直接父实体，也就是当前的 owner
        let owner = child_of.parent();

        cooldown.timer.tick(time.delta());

        let origin = transform.translation().truncate();
        let nearest_target = find_nearest_target(
            origin,
            trigger_range.range,
            vec![owner, weapon],
            &spatial_query,
            &colliders,
        );

        if let Some((_entity, target, _distance)) = nearest_target {
            let Some(direction) = (target - origin).try_normalize() else {
                // 玩家和目标完全重合，本次不开火
                continue;
            };

            if cooldown.timer.just_finished() {
                writer.write(FireWeaponMessage {
                    owner,
                    weapon,
                    origin,
                    direction,
                    target: None,
                });
            }
        }
    }
}

/// 在指定范围内查找距离 `origin` 最近的敌人。
///
/// 该函数首先查询范围内的碰撞体，
/// 然后计算 `origin` 到每个碰撞体表面的最近点，并选择距离最短的目标。
///
/// # 参数
///
/// - `origin`：搜索中心，同时也是计算距离的起点。
/// - `radius`：搜索半径。
/// - `excluded`：需要排除的实体。
/// - `spatial_query`：用于查询范围内碰撞体的空间查询器。
/// - `colliders`：用于获取碰撞体的位置和旋转信息。
///
/// # 返回值
///
/// 找到目标时返回 `Some((entity, point, distance))`：
///
/// - `entity`：最近的实体。
/// - `point`：碰撞体表面距离 `origin` 最近的点。
/// - `distance`：`origin` 到该点的直线距离。
///
/// 如果范围内没有符合条件的实体，则返回 `None`。
///
/// 没有对应碰撞体、位置或旋转数据的实体会被忽略。
fn find_nearest_target(
    origin: Vec2,
    radius: f32,
    excluded: Vec<Entity>,
    spatial_query: &SpatialQuery,
    colliders: &Query<(&Collider, &Position, &Rotation), With<Enemy>>,
) -> Option<(Entity, Vec2, f32)> {
    spatial_query
        .shape_intersections(
            &Collider::circle(radius),
            origin,
            0.0,
            &SpatialQueryFilter::default(),
        )
        .into_iter()
        .filter(|entity| !excluded.contains(entity))
        .filter_map(|entity| {
            let (collider, position, rotation) = colliders.get(entity).ok()?;

            let (projection, _is_inside) =
                collider.project_point(*position, *rotation, origin, true);

            let distance_squared = origin.distance_squared(projection);

            Some((entity, projection, distance_squared))
        })
        .min_by(|a, b| a.2.total_cmp(&b.2))
        .map(|(entity, projection, distance_squared)| (entity, projection, distance_squared.sqrt()))
}

pub fn attack(
    In((owner, fire, faction)): In<(Entity, FireWeaponMessage, Faction)>,
    mut commands: Commands,
    assets: Res<GameMeshAssets>,
) {
    spawn_projectile(
        &mut commands,
        owner,
        faction,
        fire.origin,
        fire.direction,
        &assets,
    );
}
