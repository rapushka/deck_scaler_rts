use crate::prelude::*;

pub use feature::*;
use crate::gameplay::unit::movement::UnitMovementPlugin;
use crate::gameplay::unit::view::*;

mod feature;
mod view;
mod movement;
mod stats;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UnitViewPlugin)
            .add_plugins(UnitMovementPlugin)

            .add_event::<SpawnUnit>()

            .add_systems(OnEnter(AppState::Gameplay), test_require_spawn_unit)
            .add_systems(Update, spawn_unit.run_if(in_state(AppState::Gameplay)))
        ;
    }
}