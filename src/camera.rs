use crate::input::movement::CameraMovementInput;
use crate::input::PlayerInput;
use crate::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)

            .add_systems(Update, move_camera.in_set(Order::View))
        ;
    }
}
fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2d)
        .insert(MovementSpeed(100.0))
    ;
}

fn move_camera(
    inputs: Query<&CameraMovementInput, With<PlayerInput>>,
    mut cameras: Query<(&mut Transform, &MovementSpeed), With<Camera2d>>,
    time: Res<Time<Virtual>>,
) {
    for CameraMovementInput(movement) in inputs.iter() {
        for (mut transform, speed) in cameras.iter_mut() {
            let position = &mut transform.translation;
            let scaled_speed = speed.0 * time.delta_secs();

            let movement = movement * scaled_speed;

            *position += movement.extend(0.0);
        }
    }
}