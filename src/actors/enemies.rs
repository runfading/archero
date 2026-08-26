pub mod config;
mod goblin_warrior;

use crate::actors::enemies::config::EnemyConfig;
use crate::actors::player::Player;
use crate::asset::GameMeshAssets;
use crate::core::health::{DeathMessage, Health};
use crate::core::{Faction, MoveSpeed, RunEntity, RunStats};
use crate::{GameSet, GameState, RunPhase, RunSet};
use bevy::ecs::VariantDefaults;
use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Component, Debug, Default, Clone)]
#[require(RunEntity, Faction::Enemy)]
pub struct Enemy;

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, VariantDefaults)]
#[require(Enemy)]
pub enum EnemyRank {
    #[default]
    Normal,
    Elite,
    Boss,
}

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, VariantDefaults)]
#[require(Enemy)]
pub enum EnemyArchetype {
    /// 接近玩家后攻击
    #[default]
    Melee,
    /// 保持距离并发射投射物
    Ranged,
    /// 释放法术或范围技能
    Caster,
    /// 治疗、强化或召唤其他敌人
    Support,
}

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, VariantDefaults)]
#[require(Enemy)]
pub enum EnemyClass {
    /// 战士
    #[default]
    Warrior,
    /// 弓手
    Archer,
    /// 法师
    Mage,
    /// 牧师
    Priest,
}

/// 敌人唯一标识
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[require(Enemy)]
pub enum EnemyId {
    /// 哥布林战士
    #[require(EnemyArchetype::Melee, EnemyClass::Warrior, EnemyRank::Normal)]
    GoblinWarrior,
    /// 骷髅弓箭手
    #[require(EnemyArchetype::Ranged, EnemyClass::Archer, EnemyRank::Normal)]
    SkeletonArcher,
    /// 火法师
    #[require(EnemyArchetype::Ranged, EnemyClass::Mage, EnemyRank::Elite)]
    FireMage,
    /// 治疗牧师
    #[require(EnemyArchetype::Ranged, EnemyClass::Priest, EnemyRank::Elite)]
    HealingPriest,
}

/// 敌人生成函数。每种敌人可以生成不同的 Bundle，但都直接将实体写入 World。
pub type SpawnEnemyFn = fn(&mut Commands, &EnemyConfig, &GameMeshAssets, Vec2) -> Entity;

/// 路由注册
pub struct EnemySpawnRegister {
    /// 敌人唯一标识。
    pub enemy_id: EnemyId,
    /// 生成对应敌人的函数。
    pub spawn_fn: SpawnEnemyFn,
}

inventory::collect!(EnemySpawnRegister);

static SPAWN_ENEMY_MAP: OnceLock<HashMap<EnemyId, SpawnEnemyFn>> = OnceLock::new();

pub fn spawn_enemy(
    commands: &mut Commands,
    config: &EnemyConfig,
    assets: &GameMeshAssets,
    positon: Vec2,
) {
    let map = SPAWN_ENEMY_MAP.get_or_init(|| {
        let mut map = HashMap::new();

        for registrar in inventory::iter::<EnemySpawnRegister> {
            map.insert(registrar.enemy_id, registrar.spawn_fn);
        }
        map
    });

    let Some(spawn_fn) = map.get(&config.id).copied() else {
        warn!("未实现该敌人的生成函数");
        return;
    };

    spawn_fn(commands, config, assets, positon);
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            enemy_ai.in_set(GameSet::Core).in_set(RunSet::Playing),
        );
    }
}

fn enemy_ai(
    _commands: Commands,
    time: Res<Time>,
    _assets: Res<GameMeshAssets>,
    player_pos: Query<&Transform, With<Player>>,
    mut enemies: Query<(&mut MoveSpeed, &mut Transform), (With<Enemy>, Without<Player>)>,
) {
    let player_pos = match player_pos.single() {
        Ok(player_pos) => player_pos.translation.truncate(),
        Err(err) => {
            error!("玩家唯一性有问题:{}", err);
            return;
        }
    };

    // enemy 移动
    for (mut speed, mut transform) in &mut enemies {
        // 2d游戏 v3 => v2
        let pos = transform.translation.truncate();

        // 距离
        let to_player = player_pos - pos;
        let dist = to_player.length();

        // 方向
        let dir = if dist > 0.001 {
            to_player / dist
        } else {
            Vec2::X
        };

        let mut move_dir = Vec2::ZERO;

        move_dir = dir;

        let dt = time.delta_secs();

        transform.translation.x += move_dir.x * speed.0 * dt;
        transform.translation.y += move_dir.y * speed.0 * dt;
    }
}
