use crate::prelude::*;

pub use movement::*;
pub use picking::*;
pub use destroy_after::*;

mod movement;
mod picking;
mod destroy_after;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MovementPlugin)
            .add_plugins(PickingPlugin)
            .add_plugins(DestroyAfterDelayPlugin)
        ;
    }
}