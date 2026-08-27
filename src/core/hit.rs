use bevy::prelude::*;

#[derive(Message)]
pub struct HitMessage {
    pub source: Entity,
    pub target: Entity,
    pub attack: Entity,
}
