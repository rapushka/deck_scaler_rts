use crate::prelude::*;

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Side {
    Player,
    Enemy,
}

impl Side {
    pub fn flip(&self) -> Self {
        match self {
            Side::Player => Side::Enemy,
            Side::Enemy => Side::Player,
        }
    }
}

#[derive(Component)]
pub struct OnPlayerSide;

#[derive(Component)]
pub struct OnEnemySide;