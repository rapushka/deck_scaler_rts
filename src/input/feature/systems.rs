use bevy::window::PrimaryWindow;
use crate::input::camera_movement::CameraMovementInput;
use crate::input::{bindings, CursorPosition, JustClickedOrder, JustClickedSelect, PlayerInput, PrevCursorPosition};
use crate::prelude::*;

pub fn init_input(
    mut commands: Commands,
) {
    commands.spawn(Name::from("input"))
        .insert(PlayerInput)
        .insert(CursorPosition(Vec2::ZERO))
        .insert(PrevCursorPosition(Vec2::ZERO))
        .insert(CameraMovementInput::new())
    ;
}

pub fn update_previous_cursor_position(
    mut cursors: Query<(&mut PrevCursorPosition, &CursorPosition), With<PlayerInput>>,
) {
    for (mut previous_position, current_position) in cursors.iter_mut() {
        previous_position.0 = current_position.0;
    }
}

pub fn update_cursor_position(
    mut cursors: Query<&mut CursorPosition, With<PlayerInput>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    for mut input_position in cursors.iter_mut() {
        for (camera, camera_transform) in cameras.iter() {
            for window in windows.iter() {
                let Some(cursor_position) = window.cursor_position() else {
                    continue;
                };

                let Ok(world_position) = camera.viewport_to_world(camera_transform, cursor_position) else {
                    continue;
                };

                let position = world_position.origin.truncate();

                input_position.0 = position;
            }
        }
    }
}

pub fn update_cursor_selection_click(
    mut commands: Commands,
    cursors: Query<Entity, With<PlayerInput>>,
    input: Res<ButtonInput<MouseButton>>,
) {
    for cursor in cursors.iter() {
        let is_clicked = input.just_pressed(bindings::SELECT);

        if is_clicked {
            commands.entity(cursor).insert(JustClickedSelect);
        } else {
            commands.entity(cursor).remove::<JustClickedSelect>();
        }
    }
}
pub fn update_cursor_order_click(
    mut commands: Commands,
    cursors: Query<Entity, With<PlayerInput>>,
    input: Res<ButtonInput<MouseButton>>,
) {
    for cursor in cursors.iter() {
        let is_clicked = input.just_pressed(bindings::ORDER_TARGET);

        if is_clicked {
            commands.entity(cursor).insert(JustClickedOrder);
        } else {
            commands.entity(cursor).remove::<JustClickedOrder>();
        }
    }
}