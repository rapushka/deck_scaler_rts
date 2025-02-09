use crate::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Sparkle(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SparkleChargeDuration(pub f32);
