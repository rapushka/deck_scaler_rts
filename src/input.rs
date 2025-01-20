use crate::prelude::*;

pub use feature::*;

mod feature;
pub mod bindings;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_input)

            .add_systems(Update, (
                update_cursor_position,
                update_cursor_selection_click,
                update_cursor_order_click,
            ).in_set(Order::Input))
        ;
    }
}