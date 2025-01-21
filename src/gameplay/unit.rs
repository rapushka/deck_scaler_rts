use crate::prelude::*;

use crate::gameplay::unit::view::*;
use crate::input::{CursorPosition, JustClickedSelect, PlayerInput};
pub use feature::*;
use stats::*;
pub use selection::*;
use crate::gameplay::unit::attack::AttackPlugin;
use crate::gameplay::unit::auto_mode::*;
use crate::gameplay::unit::opponent::*;
use crate::gameplay::unit::side::*;

mod feature;
mod view;
mod stats;
pub mod selection;
mod side;
mod opponent;
pub mod auto_mode;
mod attack;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UnitViewPlugin)
            .add_plugins(StatsPlugin)
            .add_plugins(UnitSelectionPlugin)
            .add_plugins(UnitSidePlugin)
            .add_plugins(OpponentPlugin)
            .add_plugins(AutoAttackPlugin)
            .add_plugins(AttackPlugin)

            .add_event::<SpawnUnit>()

            .add_systems(OnEnter(AppState::Gameplay), test_require_spawn_unit)

            .add_systems(Update, (
                spawn_unit,
                order_target_position,
            ).run_if(in_state(AppState::Gameplay)).in_set(Order::GameLogic))
        ;
    }
}