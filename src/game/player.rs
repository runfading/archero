use crate::asset::{GameAssets, GameMeshAssets};
use crate::game::health::Health;
use crate::game::{Faction, RunEntity};
use crate::{GameSet, GameState, RunPhase};
use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use bevy::scene::SceneFunction;
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

const PLAYER_SPEED: f32 = 240.0;

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
pub struct Player {}

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
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(
                Update,
                movement
                    .run_if(in_state(GameState::InGame))
                    .run_if(in_state(RunPhase::Playing))
                    .in_set(GameSet::Gameplay),
            );
    }
}

fn movement(
    time: Res<Time>,
    mut query: Query<(&ActionState<Action>, &mut Transform), With<Player>>,
) {
    let (action_state, mut transform) = match query.single_mut() {
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

    transform.translation += (direction * PLAYER_SPEED * time.delta_secs()).extend(0.0);
}

pub fn spawn_player(commands: &mut Commands, assets: &GameMeshAssets) {
    commands
        .spawn_scene(bsn! {
            Player {}
            Health::full(60.0)
        })
        .insert((
            Mesh2d(assets.circle.clone()),
            MeshMaterial2d(assets.mat_player.clone()),
            Transform::from_xyz(0.0, 0.0, 1.).with_scale(Vec3::splat(14.0)),
        ));
}
