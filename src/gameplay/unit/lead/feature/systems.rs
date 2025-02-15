use crate::gameplay::unit::lead::Lead;
use crate::prelude::*;

pub fn select_lead_on_start(
    mut commands: Commands,
    leads: Query<Entity, With<Lead>>,
) {
    for lead in leads.iter() {
        commands.entity(lead)
            .insert(SelectedUnit)
        ;
    }
}