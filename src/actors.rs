use crate::actors::enemies::EnemyPlugin;
use crate::actors::player::PlayerPlugin;
use bevy::prelude::*;

pub mod enemies;
pub mod player;

pub struct ActorsPlugin;

impl Plugin for ActorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin).add_plugins(EnemyPlugin);
    }
}
