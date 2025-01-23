use crate::input::bindings;
use crate::input::unit_control::{SetAutoUnitStateRequest, SetManualUnitStateRequest};
use crate::prelude::*;

pub fn emit_state_change_input_to_auto(
    input: Res<ButtonInput<KeyCode>>,
    mut event: EventWriter<SetAutoUnitStateRequest>,
) {
    let is_pressed = input.just_pressed(bindings::AUTO_STATE);

    if is_pressed {
        event.send(SetAutoUnitStateRequest);
    }
}

pub fn emit_state_change_input_to_manual(
    input: Res<ButtonInput<KeyCode>>,
    mut event: EventWriter<SetManualUnitStateRequest>,
) {
    let is_pressed = input.just_pressed(bindings::MANUAL_STATE);

    if is_pressed {
        event.send(SetManualUnitStateRequest);
    }
}