use crate::prelude::*;

pub use movement::*;
pub use picking::*;

mod movement;
mod picking;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MovementPlugin)
            .add_plugins(PickingPlugin)
        ;
    }
}