use crate::prelude::*;

pub use feature::*;
use crate::input::movement::InputMovementPlugin;
pub use crate::input::unit_control::*;

mod feature;
pub mod bindings;

mod unit_control;
pub mod movement;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UnitControlInputPlugin)
            .add_plugins(InputMovementPlugin)

            .add_systems(Startup, init_input)

            .add_systems(Update, (
                update_cursor_position,
                update_cursor_selection_click,
                update_cursor_order_click,
            ).in_set(Order::Input))
        ;
    }
}