use crate::prelude::*;

use view::*;
pub use feature::*;
use stats::*;
pub use selection::*;
use attack::AttackPlugin;
use status::UnitStatusPlugin;
pub use health::*;
use opponent::*;
use side::*;
pub use suit::*;
pub use defense::*;
use lead::*;
use spawn::*;
use unit_orders::UnitOrdersPlugin;
use crate::gameplay::unit::trigger::TriggerUnitPlugin;

mod feature;
mod view;

mod spawn;
mod stats;
pub mod selection;
mod side;
mod trigger;
mod opponent;
pub mod status;
mod attack;
mod defense;
mod health;
mod suit;
mod lead;
mod unit_orders;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UnitViewPlugin)
            .add_plugins(StatsPlugin)
            .add_plugins(UnitSelectionPlugin)
            .add_plugins(UnitSidePlugin)
            .add_plugins(TriggerUnitPlugin)
            .add_plugins(OpponentPlugin)
            .add_plugins(UnitStatusPlugin)
            .add_plugins(AttackPlugin)
            .add_plugins(DefensePlugin)
            .add_plugins(HealthPlugin)
            .add_plugins(SuitsPlugin)
            .add_plugins(LeadPlugin)
            .add_plugins(UnitOrdersPlugin)

            .add_event::<SpawnUnit>()

            .add_systems(OnEnter(AppState::Gameplay), test_require_spawn_unit)

            .add_systems(Update, spawn_unit
                .run_if(in_state(AppState::Gameplay))
                .in_set(Order::GameLogic),
            )
        ;
    }
}