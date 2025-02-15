use crate::prelude::*;
pub use feature::*;
use crate::input::feature::update_cursor_position;

mod feature;

pub struct InputCameraMovementPlugin;

impl Plugin for InputCameraMovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<CameraMovementInput>()

            .add_systems(Update, (
                update_wasd_movement,
                scroll_camera,
            ).chain()
                .after(update_cursor_position)
                .in_set(Order::Input))
        ;
    }
}