use crate::actors::death::DeathPlugin;
use crate::actors::enemies::EnemyPlugin;
use crate::actors::player::PlayerPlugin;
use bevy::prelude::*;
use serde::Deserialize;

mod death;
pub mod enemies;
pub mod player;

pub struct ActorsPlugin;

impl Plugin for ActorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(DeathPlugin);
    }
}
