use crate::prelude::*;

#[derive(Component)]
pub struct AttackAnimator(pub Entity);

#[derive(Component)]
pub struct PlayingAttackAnimation(pub Timer);