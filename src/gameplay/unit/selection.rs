use crate::prelude::*;

pub use feature::*;
use view::*;

mod feature;

mod view;

pub struct UnitSelectionPlugin;

impl Plugin for UnitSelectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SelectionViewPlugin)

            .add_event::<MarkUnitSelected>()

            .add_systems(Update, (
                mark_units_selected,
                deselect_old_units_on_new_selection,
                select_units,
            ).chain()
                .in_set(Order::SelectUnits))
        ;
    }
}