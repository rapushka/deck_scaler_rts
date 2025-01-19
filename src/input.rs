use crate::prelude::*;

pub use feature::*;

mod feature;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_input)

            .add_systems(Update, (
                update_cursor_position,
                update_cursor_click
            ).in_set(Order::Input))
        ;
    }
}