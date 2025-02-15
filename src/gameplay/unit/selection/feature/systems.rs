use crate::prelude::selection::feature::SelectedUnit;
use crate::prelude::*;

pub fn mark_units_selected(
    mut clicks: EventReader<Clicked>,
    units: Query<Entity, With<UnitID>>,
    mut event: EventWriter<MarkUnitSelected>,
) {
    for Clicked(target) in clicks.read() {
        let Ok(unit) = units.get(*target) else {
            continue;
        };

        event.send(MarkUnitSelected(unit));
    }
}

pub fn deselect_old_units_on_new_selection(
    mut commands: Commands,
    selected_units: Query<Entity, With<SelectedUnit>>,
    mut event: EventReader<MarkUnitSelected>,
) {
    for _any_unit_selected in event.read() {
        for unit in selected_units.iter() {
            commands.entity(unit)
                .remove::<SelectedUnit>()
            ;
        }
    }
}

pub fn select_units(
    mut commands: Commands,
    mut event: EventReader<MarkUnitSelected>,
) {
    for MarkUnitSelected(unit) in event.read() {
        commands.entity(*unit)
            .insert(SelectedUnit)
        ;
    }
}
