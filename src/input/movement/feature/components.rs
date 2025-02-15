use crate::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementInput(pub Vec2);

impl MovementInput {
    pub fn new() -> Self {
        MovementInput(Vec2::default())
    }
}

