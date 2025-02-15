use crate::common::TargetPosition;
use crate::gameplay::unit::side::feature::OnPlayerSide;
use crate::input::{CursorPosition, JustClickedOrder, PlayerInput, SetManualUnitStateRequest};
use crate::prelude::{Commands, Entity, EventWriter, Query, SelectedUnit, UnitID, With};

pub fn order_target_position(
    mut commands: Commands,
    selected_units: Query<Entity, (With<UnitID>, With<SelectedUnit>, With<OnPlayerSide>)>,
    cursors: Query<&CursorPosition, (With<PlayerInput>, With<JustClickedOrder>)>,
    mut event: EventWriter<SetManualUnitStateRequest>,
) {
    for unit in selected_units.iter() {
        for cursor_position in cursors.iter() {
            commands.entity(unit)
                .insert(TargetPosition(cursor_position.0))
            ;

            event.send(SetManualUnitStateRequest);
        }
    }
}