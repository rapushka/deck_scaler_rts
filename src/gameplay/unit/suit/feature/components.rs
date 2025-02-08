use crate::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}