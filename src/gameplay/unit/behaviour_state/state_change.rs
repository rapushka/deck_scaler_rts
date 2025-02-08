use crate::prelude::*;

pub use feature::*;

mod feature;

pub struct UnitStateChangePlugin;

impl Plugin for UnitStateChangePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                change_selected_units_to_manual,
                change_selected_units_to_auto,
            ).after(Order::UnitOrders))
        ;
    }
}
