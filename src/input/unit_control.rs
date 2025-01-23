use crate::prelude::*;

pub use feature::*;

mod feature;

pub struct UnitControlInputPlugin;

impl Plugin for UnitControlInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SetManualUnitStateRequest>()
            .add_event::<SetAutoUnitStateRequest>()

            .add_systems(Update, (
                emit_state_change_input_to_auto,
                emit_state_change_input_to_manual,
            ).in_set(Order::Input))
        ;
    }
}