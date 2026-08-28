use crate::asset::GameMeshAssets;
use crate::core::Faction;
use crate::core::attack::projectile_attack::{ProjectileSpawn, spawn_projectile};
use crate::core::attack::{AttackSpec, CombatEffect, ProjectileAttackProperty};
use crate::core::health::Health;
use crate::core::health::damage::CalDamageSnapshot;
use crate::core::property::{BaseDamage, DamageMultiplierBonus};
use crate::core::weapon::config::WeaponConfig;
use crate::core::weapon::{AimDirection, FireWeaponMessage, TargetingMode, WeaponSet};
use crate::{GameSet, RunSet};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::scene::{bsn, template_value};
use rand::RngExt;

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

    /// 冷却完成但没有目标时，限制空间查询频率。
    pub target_retry: Timer,
}

impl BowRuntime {
    pub fn new(attack_interval: f32) -> Self {
        let mut target_retry = Timer::from_seconds(0.1, TimerMode::Once);
        target_retry.finish();

        Self {
            cooldown: Timer::from_seconds(attack_interval.max(0.0), TimerMode::Once),
            target_retry,
        }
    }
}

/// 已完成通用武器校验、等待弓系统执行的攻击消息。
#[derive(Message, Debug, Clone)]
pub struct BowAttackMessage {
    pub fire: FireWeaponMessage,
    pub faction: Faction,
}

pub struct BowPlugin;
impl Plugin for BowPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<BowAttackMessage>().add_systems(
            Update,
            (
                request_weapon_fire.in_set(WeaponSet::RequestFire),
                attack.in_set(WeaponSet::Attack),
            )
                .chain()
                .in_set(GameSet::Core)
                .in_set(RunSet::Playing),
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

    let base_damage = match projectile.effect {
        CombatEffect::Damage { amount } if amount.is_finite() && amount > 0.0 => amount,
        CombatEffect::Damage { amount } => {
            warn!("弓箭基础伤害非法: {amount}");
            return;
        }
        CombatEffect::Heal { .. } => {
            warn!("弓箭攻击方式不支持治疗");
            return;
        }
        CombatEffect::ApplyStatus { .. } => {
            warn!("弓箭攻击方式暂不支持纯状态攻击");
            return;
        }
    };

    if !projectile.range.is_finite() || projectile.range <= 0.0 {
        warn!("弓箭射程非法: {}", projectile.range);
        return;
    }
    if !projectile.cooldown.is_finite() || projectile.cooldown < 0.0 {
        warn!("弓箭冷却时间非法: {}", projectile.cooldown);
        return;
    }
    if !projectile.projectile_speed.is_finite() || projectile.projectile_speed <= 0.0 {
        warn!("弓箭弹丸速度非法: {}", projectile.projectile_speed);
        return;
    }
    if !config.base_multiplying_power.is_finite() || config.base_multiplying_power < 0.0 {
        warn!("弓箭伤害倍率非法: {}", config.base_multiplying_power);
        return;
    }

    let runtime = BowRuntime::new(projectile.cooldown);
    let weapon = commands
        .spawn_scene(bsn! {
            #bow
            template_value(config.id)
            template_value(config.targeting)
            template_value(projectile)
            template_value(BowProperty::default())
            template_value(runtime)
        })
        .insert((
            Transform::default(),
            BaseDamage::new(base_damage),
            DamageMultiplierBonus::new(config.base_multiplying_power),
        ))
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
        &TargetingMode,
        &ProjectileAttackProperty,
        &mut BowRuntime,
    )>,
    owners: Query<(&Position, &Faction, Option<&AimDirection>)>,
    spatial_query: SpatialQuery,
    targets: Query<(&Collider, &Position, &Rotation, &Faction, &Health)>,
    mut writer: MessageWriter<FireWeaponMessage>,
) {
    for (weapon, child_of, targeting, projectile, mut runtime) in &mut cooldown_query {
        // 当前查询匹配到的子实体，也就是武器

        // weapon 的直接父实体，也就是当前的 owner
        let owner = child_of.parent();

        runtime.cooldown.tick(time.delta());
        runtime.target_retry.tick(time.delta());

        if !runtime.cooldown.is_finished() || !runtime.target_retry.is_finished() {
            continue;
        }

        // 以 Avian 的物理位置为权威坐标，避免读取尚未传播的 GlobalTransform。
        let Ok((owner_position, owner_faction, aim_direction)) = owners.get(owner) else {
            warn!("武器所有者缺少 Position 或 Faction: {owner:?}");
            runtime.target_retry.reset();
            continue;
        };
        let origin = owner_position.0;
        let fire_solution = match targeting {
            TargetingMode::Nearest | TargetingMode::LowestHealth => find_target(
                TargetSearch {
                    origin,
                    radius: projectile.range,
                    owner,
                    weapon,
                    owner_faction: *owner_faction,
                    targeting: *targeting,
                },
                &spatial_query,
                &targets,
            )
            .and_then(|(entity, point, _)| {
                (point - origin)
                    .try_normalize()
                    .map(|direction| (direction, Some(entity)))
            }),
            TargetingMode::ManualDirection => aim_direction
                .and_then(|aim| aim.0.try_normalize())
                .map(|direction| (direction, None)),
            TargetingMode::Random => {
                let angle = rand::rng().random_range(0.0..std::f32::consts::TAU);
                Some((Vec2::from_angle(angle), None))
            }
        };

        let Some((direction, target)) = fire_solution else {
            runtime.target_retry.reset();
            continue;
        };

        writer.write(FireWeaponMessage {
            owner,
            weapon,
            origin,
            direction,
            target,
        });
        runtime.cooldown.reset();
    }
}

/// 搜索参数
struct TargetSearch {
    origin: Vec2,
    radius: f32,
    owner: Entity,
    weapon: Entity,
    owner_faction: Faction,
    targeting: TargetingMode,
}

/// 在指定范围内按目标选择模式查找敌对单位。
///
/// 该函数首先查询范围内的碰撞体，
/// 然后计算 `origin` 到每个碰撞体表面的最近点，并按距离或生命比例选择目标。
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
fn find_target(
    search: TargetSearch,
    spatial_query: &SpatialQuery,
    targets: &Query<(&Collider, &Position, &Rotation, &Faction, &Health)>,
) -> Option<(Entity, Vec2, f32)> {
    let TargetSearch {
        origin,
        radius,
        owner,
        weapon,
        owner_faction,
        targeting,
    } = search;

    spatial_query
        .shape_intersections(
            &Collider::circle(radius),
            origin,
            0.0,
            &SpatialQueryFilter::default(),
        )
        .into_iter()
        .filter(|entity| *entity != owner && *entity != weapon)
        .filter_map(|entity| {
            let (collider, position, rotation, faction, health) = targets.get(entity).ok()?;
            if *faction == owner_faction || health.current <= 0.0 {
                return None;
            }

            let (projection, _is_inside) =
                collider.project_point(*position, *rotation, origin, true);

            let distance_squared = origin.distance_squared(projection);

            Some((entity, projection, distance_squared, health.ratio()))
        })
        .min_by(|a, b| match targeting {
            TargetingMode::LowestHealth => a.3.total_cmp(&b.3).then_with(|| a.2.total_cmp(&b.2)),
            _ => a.2.total_cmp(&b.2),
        })
        .map(|(entity, projection, distance_squared, _)| {
            (entity, projection, distance_squared.sqrt())
        })
}

fn attack(
    mut commands: Commands,
    mut messages: MessageReader<BowAttackMessage>,
    bows: Query<(&BowProperty, &ProjectileAttackProperty)>,
    assets: Res<GameMeshAssets>,
    mut snapshot_writer: MessageWriter<CalDamageSnapshot>,
) {
    for message in messages.read() {
        let fire = &message.fire;
        let Ok((bow, projectile)) = bows.get(fire.weapon) else {
            warn!("开火的弓实体缺少属性: {:?}", fire.weapon);
            continue;
        };

        for direction in
            projectile_directions(fire.direction, bow.projectile_count, bow.spread_degrees)
        {
            let origin = fire.origin + direction * PROJECTILE_SPAWN_OFFSET;
            let Some(projectile_entity) = spawn_projectile(
                &mut commands,
                fire,
                ProjectileSpawn {
                    faction: message.faction,
                    origin,
                    direction,
                    speed: projectile.projectile_speed,
                    range: projectile.range,
                    pierce: bow.pierce,
                },
                &assets,
            ) else {
                continue;
            };

            snapshot_writer.write(CalDamageSnapshot {
                owner: fire.owner,
                owner_weapon: Some(fire.weapon),
                source: projectile_entity,
            });
        }
    }
}

/// 弹道方向
///
/// 根据数量与扩散度生成本次弹道实体
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
