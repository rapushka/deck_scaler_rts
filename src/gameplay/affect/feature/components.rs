use crate::gameplay::affect::types::AffectType;
use crate::prelude::*;

#[derive(Component)]
pub struct Affect(pub AffectType);

#[derive(Component)]
pub struct AffectValue(pub f32);

#[derive(Component)]
pub struct Sender(pub Entity);

#[derive(Component)]
pub struct Target(pub Entity);
