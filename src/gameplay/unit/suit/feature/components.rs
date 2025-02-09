use crate::prelude::*;

#[derive(Component, Reflect, Copy, Clone)]
#[reflect(Component)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[derive(Component)]
pub struct Spades;

#[derive(Component)]
pub struct Hearts;

#[derive(Component)]
pub struct Clubs;

#[derive(Component)]
pub struct Diamonds;