use crate::prelude::*;

pub use feature::*;

mod feature;

pub struct UnitControlInputPlugin;

impl Plugin for UnitControlInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ToggleAttackStatus>()

            .add_systems(Update, (
                emit_toggle_attack_status,
            ).in_set(Order::Input))
        ;
    }
}