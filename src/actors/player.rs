pub mod config;

use crate::actors::player::config::PlayerConfig;
use crate::asset::GameMeshAssets;
use crate::core::health::{DeathMessage, Health};
use crate::core::{Faction, RunEntity, RunStats};
use crate::{GameSet, GameState, RunPhase, RunSet};
use avian2d::prelude::{Collider, CollisionEventsEnabled, RigidBody};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    // Movement
    Up,
    Down,
    Left,
    Right,
    // Abilities
    Ability1,
    Ultimate,
}

impl Action {
    // Lists like this can be very useful for quickly matching subsets of actions
    const DIRECTIONS: [Self; 4] = [Action::Up, Action::Down, Action::Left, Action::Right];

    fn direction(self) -> Option<Dir2> {
        match self {
            Action::Up => Some(Dir2::Y),
            Action::Down => Some(Dir2::NEG_Y),
            Action::Left => Some(Dir2::NEG_X),
            Action::Right => Some(Dir2::X),
            _ => None,
        }
    }
}

#[derive(Component, Debug, Default, Clone)]
#[require(RunEntity, InputMap<Action> = Player::default_input_map(), Faction::Player)]
pub struct Player {
    pub attack_timer: f32,
    pub move_speed: f32,
    pub level: u32,
    pub xp: u32,
    pub xp_to_next: u32,
}

impl Player {
    fn default_input_map() -> InputMap<Action> {
        use Action::*;
        let mut input_map = InputMap::default();

        input_map.insert(Up, KeyCode::ArrowUp);
        input_map.insert(Up, KeyCode::KeyW);
        input_map.insert(Up, GamepadButton::DPadUp);

        input_map.insert(Down, KeyCode::ArrowDown);
        input_map.insert(Down, KeyCode::KeyS);
        input_map.insert(Down, GamepadButton::DPadDown);

        input_map.insert(Left, KeyCode::ArrowLeft);
        input_map.insert(Left, KeyCode::KeyA);
        input_map.insert(Left, GamepadButton::DPadLeft);

        input_map.insert(Right, KeyCode::ArrowRight);
        input_map.insert(Right, KeyCode::KeyD);
        input_map.insert(Right, GamepadButton::DPadRight);

        input_map.insert(Ability1, KeyCode::KeyQ);
        input_map.insert(Ability1, GamepadButton::West);
        input_map.insert(Ability1, MouseButton::Left);

        input_map.insert(Ultimate, KeyCode::KeyR);
        input_map.insert(Ultimate, GamepadButton::LeftTrigger2);

        input_map
    }

    fn initialize(player_config: &PlayerConfig) -> Self {
        Self {
            attack_timer: player_config.attack_interval,
            move_speed: player_config.move_speed,
            level: 1,
            xp: 0,
            xp_to_next: 10,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(
                Update,
                movement.in_set(GameSet::Core).in_set(RunSet::Playing),
            );
    }
}

fn movement(
    time: Res<Time>,
    mut query: Query<(&ActionState<Action>, &mut Transform, &Player), With<Player>>,
) {
    let (action_state, mut transform, player) = match query.single_mut() {
        Ok(query) => query,
        Err(err) => {
            error!("更新玩家坐标有问题：{}", err);
            return;
        }
    };

    let direction = Action::DIRECTIONS
        .iter()
        .filter(|action| action_state.pressed(action))
        .filter_map(|action| action.direction())
        .fold(Vec2::ZERO, |sum, direction| sum + *direction)
        .normalize_or_zero();

    transform.translation += (direction * player.move_speed * time.delta_secs()).extend(0.0);
}

pub fn spawn_player(
    commands: &mut Commands,
    assets: &GameMeshAssets,
    player_config: &PlayerConfig,
) {
    let player = Player::initialize(player_config);
    commands
        .spawn_scene(bsn! {
            #Player
            template_value(player)
        })
        .insert((
            Health::full(player_config.base_hp),
            RigidBody::Dynamic,
            Collider::rectangle(1.0, 1.0),
            CollisionEventsEnabled,
            Mesh2d(assets.circle.clone()),
            MeshMaterial2d(assets.mat_player.clone()),
            Transform::from_xyz(0.0, 0.0, 1.).with_scale(Vec3::splat(14.0)),
        ));
}
