use crate::game::RunEntity;
use bevy::prelude::*;

#[derive(Component, Debug, Default, Clone)]
#[require(RunEntity)]
pub struct Player {}
