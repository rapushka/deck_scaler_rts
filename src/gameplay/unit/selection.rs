use crate::gameplay::unit::selection::feature::SelectedUnit;
use crate::prelude::*;

pub mod feature;

pub struct UnitSelectionPlugin;

impl Plugin for UnitSelectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, mark_clicked_units_as_selected.in_set(Order::Preprocess))
        ;
    }
}

pub fn mark_clicked_units_as_selected(
    mut commands: Commands,
    mut events: EventReader<Clicked>,
    units: Query<Entity, With<UnitID>>,
) {
    for Clicked(target) in events.read() {
        let Ok(unit) = units.get(*target) else {
            continue;
        };

        commands.entity(unit)
            .insert(SelectedUnit)
        ;
    }
}