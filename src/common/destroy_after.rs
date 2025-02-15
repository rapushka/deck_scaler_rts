use crate::prelude::*;
pub use feature::*;

mod feature {
    pub use components::*;
    pub(super) use systems::*;

    mod components;
    mod systems;
}

pub struct DestroyAfterDelayPlugin;

impl Plugin for DestroyAfterDelayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, destroy_after_delay.in_set(Order::Cleanups))
        ;
    }
}