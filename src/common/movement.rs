use crate::common::movement::feature::handle_movement;
use crate::prelude::*;

pub use feature::*;

mod feature;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, handle_movement.in_set(Order::Postprocess))
        ;
    }
}