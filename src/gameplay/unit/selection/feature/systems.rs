use crate::gameplay::unit::side::feature::OnPlayerSide;
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