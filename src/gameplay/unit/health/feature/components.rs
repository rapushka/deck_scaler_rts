use crate::prelude::*;

#[derive(Component)]
pub struct MaxHealth(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Health(pub f32);