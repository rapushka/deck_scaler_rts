use crate::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HealthBar(pub Entity);