use crate::common::TargetPosition;
use crate::gameplay::unit::opponent::Opponent;
use crate::gameplay::unit::side::feature::{OnEnemySide, OnPlayerSide};
use crate::input::*;
use crate::prelude::*;
use crate::prelude::status::*;

pub fn order_target_position(
    mut commands: Commands,
    selected_units: Query<Entity, (With<UnitID>, With<SelectedUnit>, With<OnPlayerSide>)>,
    mut event: EventReader<ClickTargetPosition>,
) {
    for ClickTargetPosition(position) in event.read() {
        for unit in selected_units.iter() {
            commands.entity(unit)
                .insert(TargetPosition(*position))
                .remove::<AutoAttackInRange>()
                .remove::<Opponent>()
            ;
        }
    }
}

pub fn order_attack_enemy(
    mut commands: Commands,
    selected_units: Query<Entity, (With<UnitID>, With<SelectedUnit>, With<OnPlayerSide>)>,
    mut event: EventReader<ClickTargetUnit>,
) {
    for selected_unit in selected_units.iter() {
        for ClickTargetUnit(target_unit) in event.read() {
            commands.entity(selected_unit)
                .insert(Opponent(*target_unit))
                .remove::<AutoAttackInRange>()
            ;
        }
    }
}