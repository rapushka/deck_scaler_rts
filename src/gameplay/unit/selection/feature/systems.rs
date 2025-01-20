use crate::gameplay::unit::side::feature::OnPlayerSide;
use crate::input::{CursorPosition, JustClickedSelect, PlayerInput};
use crate::prelude::*;
use crate::prelude::selection::feature::SelectedUnit;

pub fn mark_clicked_units_as_selected(
    mut commands: Commands,
    mut events: EventReader<Clicked>,
    units: Query<Entity, (With<UnitID>, With<OnPlayerSide>)>,
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

pub fn deselect_current_units_on_new_selection(
    mut commands: Commands,
    selected_units: Query<(Entity), With<SelectedUnit>>,
    clicks: Query<&CursorPosition, (With<PlayerInput>, With<JustClickedSelect>)>,
) {
    for _click in clicks.iter() {
        for unit in selected_units.iter() {
            commands.entity(unit)
                .remove::<SelectedUnit>()
            ;
        }
    }
}