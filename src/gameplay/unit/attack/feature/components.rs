use crate::prelude::*;

#[derive(Component)]
pub struct Range(pub f32);

#[derive(Component)]
pub struct AttackChargeDuration(pub f32);

#[derive(Component)]
pub struct ChargingAttack(pub Timer);