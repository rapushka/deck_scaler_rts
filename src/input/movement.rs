use crate::prelude::*;
pub use feature::*;

mod feature;

pub struct InputMovementPlugin;

impl Plugin for InputMovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<MovementInput>()

            .add_systems(Update, (
                update_movement,
            ).in_set(Order::Input))
        ;
    }
}