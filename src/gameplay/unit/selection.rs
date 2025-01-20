use crate::gameplay::unit::selection::feature::SelectedUnit;
use crate::prelude::*;
use crate::prelude::selection::feature::mark_clicked_units_as_selected;
use crate::prelude::selection::view::SelectionViewPlugin;

pub mod feature;

mod view;

pub struct UnitSelectionPlugin;

impl Plugin for UnitSelectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SelectionViewPlugin)

            .add_systems(Update, mark_clicked_units_as_selected.in_set(Order::Preprocess))
        ;
    }
}