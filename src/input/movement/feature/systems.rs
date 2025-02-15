use input::bindings;
use crate::input;
use crate::input::movement::MovementInput;
use crate::input::PlayerInput;
use crate::prelude::*;

pub fn update_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut entities: Query<&mut MovementInput, With<PlayerInput>>,
) {
    for mut movement in entities.iter_mut() {
        let mut tmp = Vec2::ZERO;

        if input.pressed(bindings::MOVE_UP) {
            tmp.y += 1.0;
        }
        if input.pressed(bindings::MOVE_DOWN) {
            tmp.y -= 1.0;
        }
        if input.pressed(bindings::MOVE_RIGHT) {
            tmp.x += 1.0;
        }
        if input.pressed(bindings::MOVE_LEFT) {
            tmp.x -= 1.0;
        }

        movement.0 = tmp.normalize_or_zero();
    }
}
