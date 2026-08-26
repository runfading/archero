use crate::actors::enemies::Enemy;
use crate::actors::player::Player;
use crate::core::RunStats;
use crate::core::health::{DeathMessage, Health};
use crate::{GameSet, GameState, RunPhase, RunSet};
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
    players: Query<(), With<Player>>,
    query: Query<&Name>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for death in deaths.read() {
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
            commands.entity(death.entity).despawn();
        }

        if players.contains(death.entity) {
            next_state.set(GameState::GameOver);
            continue;
        }
    }
}
