use crate::actors::enemies::Enemy;
use crate::actors::player::Player;
use crate::core::RunStats;
use crate::core::health::DeathMessage;
use crate::{GameSet, GameState, RunSet};
use bevy::prelude::*;

pub struct DeathPlugin;

impl Plugin for DeathPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, death.in_set(GameSet::Core).in_set(RunSet::Playing));
    }
}

fn death(
    mut commands: Commands,
    mut deaths: MessageReader<DeathMessage>,
    mut stats: ResMut<RunStats>,
    enemies: Query<(), With<Enemy>>,
    mut players: Query<&mut Player>,
    query: Query<&Name>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for death in deaths.read() {
        let player_died = players.contains(death.entity);
        let kill_name = if let Ok(name) = query.get(death.killer) {
            name.to_string()
        } else {
            "".to_string()
        };

        let death_name = if let Ok(name) = query.get(death.entity) {
            name.to_string()
        } else {
            "".to_string()
        };

        info!("killer:{kill_name},death：{death_name}");
        if enemies.contains(death.entity) {
            stats.kills += 1;
            stats.gold += 1;
            if let Ok(mut player) = players.single_mut() {
                player.gain_xp(Player::XP_PER_KILL);
            }
            commands.entity(death.entity).despawn();
        }

        if player_died {
            next_state.set(GameState::GameOver);
            continue;
        }
    }
}
