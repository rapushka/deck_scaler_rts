use crate::prelude::*;

use crate::gameplay::unit::view::*;
use crate::input::{CursorPosition, JustClickedSelect, PlayerInput};
pub use feature::*;
use stats::*;
pub use selection::*;
use crate::gameplay::unit::side::UnitSidePlugin;

mod feature;
mod view;
mod stats;
pub mod selection;
mod side;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UnitViewPlugin)
            .add_plugins(StatsPlugin)
            .add_plugins(UnitSelectionPlugin)
            .add_plugins(UnitSidePlugin)

            .add_event::<SpawnUnit>()

            .add_systems(OnEnter(AppState::Gameplay), test_require_spawn_unit)
            .add_systems(Update, spawn_unit.run_if(in_state(AppState::Gameplay)).in_set(Order::GameLogic))

            .add_systems(Update, test_target_position.after(spawn_unit))
        ;
    }
}