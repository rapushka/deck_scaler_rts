use bevy::window::PrimaryWindow;
use crate::input::camera_movement::CameraMovementInput;
use crate::input::*;
use crate::prelude::*;

pub fn init_input(
    mut commands: Commands,
) {
    commands.spawn(Name::from("input"))
        .insert(PlayerInput)
        .insert(CursorPosition(Vec2::ZERO))
        .insert(CursorScreenPosition::new())
        .insert(CameraMovementInput::new())
    ;
}

pub fn update_cursor_screen_positions(
    mut cursors: Query<&mut CursorScreenPosition, With<PlayerInput>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    for mut cursor in cursors.iter_mut() {
        for window in windows.iter() {
            let Some(screen_position) = window.cursor_position() else {
                continue;
            };

            cursor.previous = cursor.current;
            cursor.current = screen_position;
        }
    }
}

pub fn update_cursor_world_position(
    mut cursors: Query<(&mut CursorPosition, &CursorScreenPosition), With<PlayerInput>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    for (mut input_position, screen_position) in cursors.iter_mut() {
        for (camera, camera_transform) in cameras.iter() {
            let screen_position = screen_position.current;
            let Ok(world_position) = camera.viewport_to_world(camera_transform, screen_position) else {
                continue;
            };

            input_position.0 = world_position.origin.truncate();
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