use crate::prelude::*;

pub use feature::*;

mod feature;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameplay), test_require_spawn_unit)

            .add_systems(Update, spawn_unit.run_if(in_state(AppState::Gameplay)))
        ;
    }
}