use crate::actors::enemies::{EnemyId, EnemySpawnRegister, EnemyTier};
use crate::asset::GameMeshAssets;
use crate::core::MoveSpeed;
use crate::core::health::Health;
use bevy::prelude::*;

inventory::submit!(EnemySpawnRegister {
    enemy_id: EnemyId::GoblinWarrior,
    spawn_fn: spawn,
});

fn spawn(commands: &mut Commands, assets: &GameMeshAssets, position: Vec2) -> Entity {
    commands
        .spawn_scene(bsn! {
            {EnemyId::GoblinWarrior.enemy_template()}
            Health::full(60.0)
            MoveSpeed(60.0)
            EnemyTier::Tier1
        })
        .insert((
            Mesh2d(assets.circle.clone()),
            MeshMaterial2d(assets.mat_melee.clone()),
            Transform::from_translation(position.extend(1.0)).with_scale(Vec3::splat(14.0)),
        ))
        .id()
}
