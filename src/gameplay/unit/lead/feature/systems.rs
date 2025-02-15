use crate::gameplay::unit::lead::Lead;
use crate::input::movement::MovementInput;
use crate::input::PlayerInput;
use crate::prelude::*;

pub fn move_lead(
    inputs: Query<&MovementInput, With<PlayerInput>>,
    mut entities: Query<(&mut Transform, &MovementSpeed), With<Lead>>,
    time: Res<Time<Virtual>>,
) {
    for MovementInput(movement) in inputs.iter() {
        for (mut transform, speed) in entities.iter_mut() {
            let position = &mut transform.translation;
            let scaled_speed = speed.0 * time.delta_secs();

            let movement = movement * scaled_speed;

            *position += movement.extend(0.0);
        }
    }
}