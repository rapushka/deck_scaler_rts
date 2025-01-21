use crate::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Opponent(pub Entity);