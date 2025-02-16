use crate::prelude::*;

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq)]
pub enum OnSide {
    Player,
    Enemy,
}

impl OnSide {
    pub fn flip(&self) -> Self {
        match self {
            OnSide::Player => OnSide::Enemy,
            OnSide::Enemy => OnSide::Player,
        }
    }
}

#[derive(Component)]
pub struct OnPlayerSide;

#[derive(Component)]
pub struct OnEnemySide;