use crate::actors::enemies::config::EnemyConfig;
use crate::actors::enemies::{EnemyId, EnemySpawnRegister};
use crate::asset::GameMeshAssets;
use crate::core::MoveSpeed;
use crate::core::health::Health;
use avian2d::prelude::{Collider, CollisionEventsEnabled, RigidBody};
use bevy::prelude::*;

inventory::submit!(EnemySpawnRegister {
    enemy_id: EnemyId::GoblinWarrior,
    spawn_fn: spawn,
});

fn spawn(
    commands: &mut Commands,
    config: &EnemyConfig,
    assets: &GameMeshAssets,
    position: Vec2,
) -> Entity {
    commands
        .spawn((
            Name("goblin_warrior".into()),
            config.id,
            RigidBody::Dynamic,
            Collider::rectangle(1.0, 1.0),
            CollisionEventsEnabled,
            Health::full(config.base_hp),
            MoveSpeed(config.move_speed),
            Mesh2d(assets.circle.clone()),
            MeshMaterial2d(assets.mat_melee.clone()),
            Transform::from_translation(position.extend(1.0)).with_scale(Vec3::splat(14.0)),
        ))
        .id()
}
