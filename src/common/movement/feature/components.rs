use crate::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TargetPosition(pub Vec2);

#[derive(Component)]
pub struct MovementSpeed(pub f32);