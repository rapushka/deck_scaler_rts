use crate::gameplay::unit::movement::feature::*;
use crate::prelude::*;
pub use feature::*;

mod feature;

pub struct UnitMovementPlugin;

impl Plugin for UnitMovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<TargetPosition>()

            .add_systems(Update, move_unit_to_target.in_set(Order::GameLogic))
        ;
    }
}