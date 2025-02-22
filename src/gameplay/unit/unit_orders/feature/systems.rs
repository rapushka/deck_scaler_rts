use crate::common::TargetPosition;
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
            ;
        }
    }
}

pub fn order_attack_enemy(
    mut commands: Commands,
    selected_units: Query<Entity, (With<UnitID>, With<SelectedUnit>, With<OnPlayerSide>)>,
    mut event: EventReader<ClickTargetUnit>,
    enemies: Query<&Transform, With<OnEnemySide>>,
) {
    for selected_unit in selected_units.iter() {
        for ClickTargetUnit(target_unit) in event.read() {
            let enemy_transform = cq!(enemies.get(*target_unit));

            commands.entity(selected_unit)
                .insert(TargetPosition(enemy_transform.translation.truncate()))
                .remove::<AutoAttackInRange>()
            ;
        }
    }
}