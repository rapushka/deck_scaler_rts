use crate::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Sparkle(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct NextSparkleCharge(pub f32);

// stats
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SparkleCapacity(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SparkleChargeRate(pub f32);
