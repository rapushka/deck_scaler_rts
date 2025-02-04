use crate::gameplay::unit::behaviour_state::auto_mode::AutoAttackState;
use crate::gameplay::unit::opponent::Opponent;
use crate::input::{SetAutoUnitStateRequest, SetManualUnitStateRequest};
use crate::prelude::*;

pub fn change_selected_units_to_auto(
    mut commands: Commands,
    selected_units: Query<Entity, (With<SelectedUnit>, Without<AutoAttackState>)>,
    mut event: EventReader<SetAutoUnitStateRequest>,
) {
    for _ in event.read() {
        for unit in selected_units.iter() {
            commands.entity(unit)
                .insert(AutoAttackState)
            ;
        }
    }
}

pub fn change_selected_units_to_manual(
    mut commands: Commands,
    selected_units: Query<Entity, (With<SelectedUnit>, With<AutoAttackState>)>,
    mut event: EventReader<SetManualUnitStateRequest>,
) {
    for _ in event.read() {
        for unit in selected_units.iter() {
            commands.entity(unit)
                .remove::<AutoAttackState>()
                .remove::<Opponent>()
            ;
        }
    }
}