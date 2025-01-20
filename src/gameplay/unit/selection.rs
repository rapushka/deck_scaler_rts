use crate::gameplay::unit::selection::feature::SelectedUnit;
use crate::prelude::*;
use crate::prelude::selection::feature::mark_clicked_units_as_selected;

pub mod feature;

pub struct UnitSelectionPlugin;

impl Plugin for UnitSelectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, mark_clicked_units_as_selected.in_set(Order::Preprocess))
        ;
    }
}