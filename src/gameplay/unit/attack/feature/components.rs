use crate::prelude::*;

// stats

#[derive(Component)]
pub struct Range(pub f32);

#[derive(Component)]
pub struct AttackChargeDuration(pub f32);

#[derive(Component)]
pub struct Damage(pub f32);

// states

#[derive(Component)]
pub struct ChargingAttack(pub Timer);

#[derive(Component)]
pub struct Attacking(pub Timer);

#[derive(Event)]
pub struct AttackCharged(pub Entity);