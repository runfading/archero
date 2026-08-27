use crate::core::Faction;
use crate::core::weapon::bow::{BowAttackMessage, BowPlugin};
use crate::{GameSet, RunSet};
use bevy::ecs::VariantDefaults;
use bevy::prelude::*;
use serde::Deserialize;

pub mod bow;
pub mod config;

#[derive(
    Deserialize, Component, Default, Debug, Copy, Clone, Hash, Eq, PartialEq, VariantDefaults,
)]
pub enum WeaponId {
    #[default]
    Bow,
}

#[derive(Deserialize, Component, Default, Debug, Copy, Clone)]
pub enum TargetingMode {
    /// 最近
    #[default]
    Nearest,
    /// 最低生命值
    LowestHealth,
    /// 手动方向
    ManualDirection,
    /// 随机方向
    Random,
}

/// 武器开火消息
#[derive(Message, Debug, Clone)]
pub struct FireWeaponMessage {
    /// 攻击发起者
    pub owner: Entity,

    /// 实际开火的武器实体
    pub weapon: Entity,

    /// 本次攻击的起点快照
    pub origin: Vec2,

    /// 本次攻击的最终方向，必须归一化
    pub direction: Vec2,

    /// 可选目标，用于追踪弹、连锁攻击等
    pub target: Option<Entity>,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum WeaponSet {
    RequestFire,
    Fire,
    Attack,
    ChangeCartridge,
}

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<FireWeaponMessage>()
            .configure_sets(
                Update,
                (
                    WeaponSet::RequestFire,
                    WeaponSet::Fire.after(WeaponSet::RequestFire),
                    WeaponSet::Attack.after(WeaponSet::Fire),
                    WeaponSet::ChangeCartridge.after(WeaponSet::Attack),
                ),
            )
            .add_plugins(BowPlugin)
            .add_systems(
                Update,
                fire.in_set(GameSet::Core)
                    .in_set(RunSet::Playing)
                    .in_set(WeaponSet::Fire),
            );
    }
}

fn fire(
    mut fire_message: MessageReader<FireWeaponMessage>,
    weapon_id_query: Query<&WeaponId>,
    faction_query: Query<&Faction>,
    mut bow_writer: MessageWriter<BowAttackMessage>,
) {
    for fire in fire_message.read() {
        let weapon = match weapon_id_query.get(fire.weapon) {
            Ok(weapon) => weapon,
            Err(err) => {
                error!("不是武器实体{:?}", err);
                continue;
            }
        };

        let faction = match faction_query.get(fire.owner) {
            Ok(faction) => faction,
            Err(err) => {
                error!("实体未区分阵营{:?}", err);
                continue;
            }
        };

        // 应用攻击逻辑
        match weapon {
            WeaponId::Bow => {
                bow_writer.write(BowAttackMessage {
                    fire: fire.clone(),
                    faction: *faction,
                });
            }
        }
    }
}
