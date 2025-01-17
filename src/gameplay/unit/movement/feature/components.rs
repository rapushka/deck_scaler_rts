use crate::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TargetPosition(pub Vec2);