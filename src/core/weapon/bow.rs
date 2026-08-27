use crate::actors::enemies::Enemy;
use crate::asset::GameMeshAssets;
use crate::core::Faction;
use crate::core::attack::projectile_attack::spawn_projectile;
use crate::core::attack::{AttackSpec, CombatEffect, ProjectileAttackProperty};
use crate::core::weapon::config::WeaponConfig;
use crate::core::weapon::{FireWeaponMessage, WeaponId, WeaponSet};
use crate::{GameSet, RunSet};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::scene::{bsn, template_value};

/// 箭矢从单位中心沿发射方向向外偏移，避免出生时与发射者重叠。
const PROJECTILE_SPAWN_OFFSET: f32 = 20.0;

#[derive(Component, Debug, Clone)]
pub struct BowProperty {
    /// 每次射出的箭矢数量
    pub projectile_count: u32,

    /// 多支箭之间的总扩散角度
    pub spread_degrees: f32,

    /// 可额外穿透的目标数量
    pub pierce: u32,

    /// 每轮连续开火次数
    pub burst_count: u32,

    /// 连续开火之间的间隔
    pub burst_interval: f32,
}

impl Default for BowProperty {
    fn default() -> Self {
        Self {
            projectile_count: 1,
            spread_degrees: 0.0,
            pierce: 0,
            burst_count: 1,
            burst_interval: 0.1,
        }
    }
}

#[derive(Component, Debug, Clone, Default)]
pub struct BowRuntime {
    /// 下次允许攻击的计时器
    pub cooldown: Timer,

    /// 当前剩余箭矢
    pub current_ammo: Option<u32>,
}

impl BowRuntime {
    pub fn new(attack_interval: f32) -> Self {
        Self {
            cooldown: Timer::from_seconds(attack_interval.max(0.0), TimerMode::Once),
            current_ammo: None,
        }
    }
}

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

pub fn spawn_bow(commands: &mut Commands, owner: Entity, config: &WeaponConfig) {
    let projectile = match &config.attack {
        AttackSpec::Projectile(property) => property.clone(),
        _ => {
            warn!("弓箭攻击方式暂时只支持弹道，不支持{:?}", config.attack);
            return;
        }
    };

    if matches!(projectile.effect, CombatEffect::Heal { .. }) {
        warn!("弓箭攻击方式不支持治疗");
        return;
    }

    let runtime = BowRuntime::new(projectile.cooldown);
    let weapon = commands
        .spawn_scene(bsn! {
            #bow
            WeaponId::Bow
            template_value(config.targeting.clone())
            template_value(projectile)
            template_value(BowProperty::default())
            template_value(runtime)
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
        &ProjectileAttackProperty,
        &mut BowRuntime,
    )>,
    owner_positions: Query<&Position>,
    spatial_query: SpatialQuery,
    // 先过滤掉敌人，后面再想办法
    colliders: Query<(&Collider, &Position, &Rotation), With<Enemy>>,
    mut writer: MessageWriter<FireWeaponMessage>,
) {
    for (weapon, child_of, projectile, mut runtime) in &mut cooldown_query {
        // 当前查询匹配到的子实体，也就是武器

        // weapon 的直接父实体，也就是当前的 owner
        let owner = child_of.parent();

        runtime.cooldown.tick(time.delta());

        if !runtime.cooldown.is_finished() {
            continue;
        }

        // 以 Avian 的物理位置为权威坐标，避免读取尚未传播的 GlobalTransform。
        let Ok(owner_position) = owner_positions.get(owner) else {
            warn!("武器所有者缺少 Position: {owner:?}");
            continue;
        };
        let origin = owner_position.0;
        let nearest_target = find_nearest_target(
            origin,
            projectile.range,
            vec![owner, weapon],
            &spatial_query,
            &colliders,
        );

        if let Some((target_entity, target, _distance)) = nearest_target {
            let Some(direction) = (target - origin).try_normalize() else {
                // 玩家和目标完全重合，本次不开火
                continue;
            };

            writer.write(FireWeaponMessage {
                owner,
                weapon,
                origin,
                direction,
                target: Some(target_entity),
            });
            runtime.cooldown.reset();
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
    commands: &mut Commands,
    owner: Entity,
    fire: &FireWeaponMessage,
    faction: Faction,
    bows: &Query<(&BowProperty, &ProjectileAttackProperty)>,
    assets: &GameMeshAssets,
) {
    let Ok((bow, projectile)) = bows.get(fire.weapon) else {
        warn!("开火的弓实体缺少属性: {:?}", fire.weapon);
        return;
    };

    let damage = match projectile.effect {
        CombatEffect::Damage { amount } => amount,
        CombatEffect::Heal { .. } => return,
    };

    for direction in projectile_directions(fire.direction, bow.projectile_count, bow.spread_degrees)
    {
        let origin = fire.origin + direction * PROJECTILE_SPAWN_OFFSET;
        spawn_projectile(
            commands,
            owner,
            faction,
            origin,
            direction,
            projectile.projectile_speed,
            damage,
            bow.pierce,
            assets,
        );
    }
}

fn projectile_directions(
    center: Vec2,
    count: u32,
    spread_degrees: f32,
) -> impl Iterator<Item = Vec2> {
    let count = count.max(1);
    let total_spread = spread_degrees.to_radians();

    (0..count).map(move |index| {
        let t = if count == 1 {
            0.5
        } else {
            index as f32 / (count - 1) as f32
        };
        let angle = -total_spread * 0.5 + total_spread * t;
        Mat2::from_angle(angle) * center
    })
}
