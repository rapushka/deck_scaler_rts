use crate::common::movement::feature::*;
use crate::prelude::*;

pub use feature::*;

mod feature;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<TargetPosition>()

            .add_systems(Update, move_to_target.in_set(Order::Postprocess))
        ;
    }
}