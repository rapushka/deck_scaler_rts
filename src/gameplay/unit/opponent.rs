use crate::prelude::*;

pub use feature::*;

mod feature;

pub struct OpponentPlugin;

impl Plugin for OpponentPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Opponent>()

            .add_systems(Update, move_to_opponent.in_set(Order::GameLogic))
        ;
    }
}