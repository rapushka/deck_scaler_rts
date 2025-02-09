use crate::prelude::*;

use view::*;
pub use feature::*;
use stats::*;
pub use selection::*;
use attack::AttackPlugin;
use behaviour_state::UnitBehaviourPlugin;
pub use health::*;
use opponent::*;
use side::*;
pub use suit::*;
pub use defense::*;
pub use sparkle::*;

mod feature;

mod view;
mod stats;
pub mod selection;
mod side;
mod opponent;
mod behaviour_state;
mod attack;
mod defense;
mod health;
mod suit;
mod sparkle;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UnitViewPlugin)
            .add_plugins(StatsPlugin)
            .add_plugins(UnitSelectionPlugin)
            .add_plugins(UnitSidePlugin)
            .add_plugins(OpponentPlugin)
            .add_plugins(UnitBehaviourPlugin)
            .add_plugins(AttackPlugin)
            .add_plugins(DefensePlugin)
            .add_plugins(HealthPlugin)
            .add_plugins(SuitsPlugin)
            .add_plugins(SparklePlugin)

            .add_event::<SpawnUnit>()

            .add_systems(OnEnter(AppState::Gameplay), test_require_spawn_unit)

            .add_systems(Update, spawn_unit
                .run_if(in_state(AppState::Gameplay))
                .in_set(Order::GameLogic),
            )

            .add_systems(Update, order_target_position
                .run_if(in_state(AppState::Gameplay))
                .in_set(Order::UnitOrders),
            )
        ;
    }
}