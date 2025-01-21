use crate::prelude::*;

#[derive(Component, Debug, Copy, Clone)]
pub enum Side {
    Player,
    Enemy,
}

#[derive(Component)]
pub struct OnPlayerSide;

#[derive(Component)]
pub struct OnEnemySide;