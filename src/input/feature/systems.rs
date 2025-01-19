use bevy::window::PrimaryWindow;
use crate::input::{CursorPosition, JustClicked, PlayerInput};
use crate::prelude::*;

pub fn init_input(
    mut commands: Commands,
) {
    commands.spawn(Name::from("input"))
        .insert(PlayerInput)
        .insert(CursorPosition(Vec2::ZERO))
    ;
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

pub fn update_cursor_click(
    mut commands: Commands,
    cursors: Query<Entity, With<PlayerInput>>,
    input: Res<ButtonInput<MouseButton>>,
) {
    for cursor in cursors.iter() {
        let is_clicked = input.just_pressed(MouseButton::Left);

        if is_clicked {
            commands.entity(cursor).insert(JustClicked);
        } else {
            commands.entity(cursor).remove::<JustClicked>();
        }
    }
}