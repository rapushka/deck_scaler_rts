use crate::prelude::*;

pub use movement::*;

mod movement;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MovementPlugin)
        ;
    }
}