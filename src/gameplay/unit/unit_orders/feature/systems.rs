use crate::common::TargetPosition;
use crate::gameplay::unit::side::feature::OnPlayerSide;
use crate::input::*;
use crate::prelude::*;
use crate::prelude::status::*;

pub fn order_target_position(
    mut commands: Commands,
    selected_units: Query<Entity, (With<UnitID>, With<SelectedUnit>, With<OnPlayerSide>)>,
    cursors: Query<&CursorWorldPosition, (With<PlayerInput>, With<JustClickedTarget>)>,
) {
    for unit in selected_units.iter() {
        for cursor_position in cursors.iter() {
            commands.entity(unit)
                .insert(TargetPosition(cursor_position.0))
                .insert(OrderMoveToPosition(cursor_position.0))
            ;
        }
    }
}

pub fn order_attack_unit(
    mut commands: Commands,
    selected_units: Query<Entity, (With<UnitID>, With<SelectedUnit>, With<OnPlayerSide>)>,
    cursors: Query<&CursorWorldPosition, (With<PlayerInput>, With<JustClickedTarget>)>,
) {
    for unit in selected_units.iter() {
        for cursor_position in cursors.iter() {
            commands.entity(unit)
                .insert(TargetPosition(cursor_position.0))
                .insert(OrderMoveToPosition(cursor_position.0))
            ;
        }
    }
}