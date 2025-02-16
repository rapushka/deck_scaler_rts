use crate::input::*;

pub fn emit_toggle_attack_status(
    input: Res<ButtonInput<KeyCode>>,
    mut event: EventWriter<ToggleAttackStatus>,
) {
    let is_pressed = input.just_pressed(bindings::TOGGLE_ATTACK);

    if is_pressed {
        event.send(ToggleAttackStatus);
    }
}