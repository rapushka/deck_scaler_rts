use input::bindings;
use crate::input;
use crate::input::camera_movement::CameraMovementInput;
use crate::input::*;
use crate::prelude::*;

pub fn update_wasd_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut inputs: Query<&mut CameraMovementInput, With<PlayerInput>>,
) {
    for mut movement in inputs.iter_mut() {
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

pub fn scroll_camera(
    input: Res<ButtonInput<MouseButton>>,
    mut cursors: Query<(&mut CameraMovementInput, &CursorScreenPosition), With<PlayerInput>>,
) {
    if !input.pressed(bindings::MOVE_CAMERA) {
        return;
    }

    for (mut movement, position) in cursors.iter_mut() {
        movement.0 = position.delta();
    }
}