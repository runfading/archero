use crate::actors::enemies::config::EnemyConfig;
use crate::actors::enemies::{EnemyId, EnemySpawnRegister};
use crate::asset::GameMeshAssets;
use crate::core::MoveSpeed;
use crate::core::attack::contact_attack::{ContactDamage, on_contact_damage};
use crate::core::health::Health;
use avian2d::prelude::{Collider, CollisionEventsEnabled, LinearVelocity, LockedAxes, RigidBody};
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
    let mut eneity = commands.spawn_scene(bsn! {
        #goblin_warrior
    });

    eneity.insert((
        config.id,
        Health::full(config.base_hp),
        MoveSpeed(config.move_speed),
        // 碰撞
        RigidBody::Dynamic,
        Collider::rectangle(1.0, 1.0),
        CollisionEventsEnabled,
        LockedAxes::ROTATION_LOCKED,
        LinearVelocity::default(),
        // 图形
        Mesh2d(assets.circle.clone()),
        MeshMaterial2d(assets.mat_melee.clone()),
        Transform::from_translation(position.extend(1.0)).with_scale(Vec3::splat(14.0)),
    ));

    if config.concat_damage.0.is_finite() && config.concat_damage.0 > 0.0 {
        eneity
            .insert(ContactDamage(config.concat_damage.0))
            .observe(on_contact_damage);
    }

    eneity.id()
}
