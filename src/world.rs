use crate::world::level::LevelPlugin;
use bevy::prelude::*;

pub mod level;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LevelPlugin);
    }
}
