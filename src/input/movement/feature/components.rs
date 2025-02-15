use crate::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CameraMovementInput(pub Vec2);

impl CameraMovementInput {
    pub fn new() -> Self {
        CameraMovementInput(Vec2::default())
    }
}

