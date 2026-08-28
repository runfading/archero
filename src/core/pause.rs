use crate::RunPhase;
use avian2d::prelude::{Physics, PhysicsTime};
use bevy::prelude::*;

pub struct PausePlugin;
impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(RunPhase::Paused), pause_physics)
            .add_systems(OnExit(RunPhase::Paused), resume_physics)
            .add_systems(OnEnter(RunPhase::LevelUp), pause_physics)
            .add_systems(OnExit(RunPhase::LevelUp), resume_physics);
    }
}

fn pause_physics(mut physics_time: ResMut<Time<Physics>>) {
    physics_time.pause();
}

fn resume_physics(mut physics_time: ResMut<Time<Physics>>) {
    physics_time.unpause();
}
