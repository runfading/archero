use crate::actors::enemies::config::EnemyConfig;
use crate::actors::enemies::{EnemyId, EnemySpawnRegister};
use crate::asset::GameMeshAssets;
use crate::core::MoveSpeed;
use crate::core::attack::contact_attack::{
    ContactDamage, ContactDamageCooldown, on_contact_damage,
};
use crate::core::health::Health;
use crate::core::weapon::bow::spawn_bow;
use crate::core::weapon::config::WeaponConfigs;
use avian2d::prelude::{Collider, CollisionEventsEnabled, LinearVelocity, LockedAxes, RigidBody};
use bevy::prelude::*;

inventory::submit!(EnemySpawnRegister {
    enemy_id: EnemyId::GoblinWarrior,
    spawn_fn: spawn,
});

fn spawn(
    commands: &mut Commands,
    config: &EnemyConfig,
    weapons: &WeaponConfigs,
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
        let cooldown = if config.contact_damage_cooldown.0.is_finite()
            && config.contact_damage_cooldown.0 >= 0.0
        {
            config.contact_damage_cooldown
        } else {
            warn!(
                "接触伤害冷却非法，使用默认值: {}",
                config.contact_damage_cooldown.0
            );
            ContactDamageCooldown::default()
        };

        eneity
            .insert((ContactDamage(config.concat_damage.0), cooldown))
            .observe(on_contact_damage);
    }

    let enemy = eneity.id();
    let Some(weapon_config) = weapons.get(config.weapon) else {
        warn!("敌人武器配置不存在: {:?}", config.weapon);
        return enemy;
    };
    spawn_bow(commands, enemy, weapon_config);

    enemy
}

#[cfg(test)]
mod tests {
    use super::spawn;
    use crate::actors::enemies::config::EnemyConfig;
    use crate::asset::GameMeshAssets;
    use crate::core::Faction;
    use crate::core::weapon::WeaponId;
    use crate::core::weapon::bow::BowRuntime;
    use crate::core::weapon::config::WeaponConfigs;
    use bevy::asset::AssetPlugin;
    use bevy::ecs::world::CommandQueue;
    use bevy::prelude::*;
    use bevy::scene::ScenePlugin;
    use crate::core::property::BaseDamage;

    fn mesh_assets() -> GameMeshAssets {
        GameMeshAssets {
            circle: Handle::default(),
            square: Handle::default(),
            mat_bg: Handle::default(),
            mat_player: Handle::default(),
            mat_melee: Handle::default(),
            mat_ranged: Handle::default(),
            mat_elite: Handle::default(),
            mat_boss: Handle::default(),
            mat_arrow: Handle::default(),
            mat_enemy_shot: Handle::default(),
            mat_coin: Handle::default(),
            mat_heart: Handle::default(),
        }
    }

    #[test]
    fn spawning_goblin_attaches_configured_bow() {
        let config: EnemyConfig = ron::from_str(
            r#"(
                id: GoblinWarrior,
                move_speed: 60.0,
                base_hp: 60.0,
                weapon: Bow,
            )"#,
        )
        .expect("敌人配置应该可以反序列化");
        let weapons: WeaponConfigs = ron::from_str(include_str!(
            "../../../assets/config/default_weapon.weapon.ron"
        ))
        .expect("武器配置应该可以反序列化");
        let assets = mesh_assets();
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default(), ScenePlugin));
        let world = app.world_mut();
        let mut queue = CommandQueue::default();
        let enemy = {
            let mut commands = Commands::new(&mut queue, world);
            spawn(&mut commands, &config, &weapons, &assets, Vec2::ZERO)
        };
        queue.apply(world);

        let children = world
            .get::<Children>(enemy)
            .expect("敌人应该挂载一个武器子实体");
        let bow = children.iter().next().expect("敌人应该拥有弓");

        assert_eq!(world.get::<Faction>(enemy), Some(&Faction::Enemy));
        assert_eq!(world.get::<WeaponId>(bow), Some(&WeaponId::Bow));
        assert!(world.get::<BowRuntime>(bow).is_some());
        assert_eq!(
            world.get::<BaseDamage>(bow).map(BaseDamage::get),
            Some(10.0)
        );
    }
}
