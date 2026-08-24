pub mod config;
mod director;
pub mod spawn;

use crate::world::level::director::LevelDirectorPlugin;
use bevy::prelude::*;
use rand::RngExt;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LevelDirectorPlugin);
    }
}
