use crate::gameplay::unit::movement::feature::*;
use crate::prelude::*;

mod feature;

pub struct UnitMovementPlugin;

impl Plugin for UnitMovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, move_unit_to_target.in_set(Order::GameLogic))
        ;
    }
}