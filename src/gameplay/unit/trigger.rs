use crate::prelude::*;
pub use feature::*;
mod feature {
    pub use components::*;
    pub(super) use systems::*;

    mod components;
    mod systems;
}

pub struct TriggerUnitPlugin;

impl Plugin for TriggerUnitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, find_opponent_in_range.after(Order::MoveUnits))
        ;
    }
}