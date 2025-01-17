use crate::prelude::*;

pub use feature::*;
use crate::gameplay::unit::movement::UnitMovementPlugin;
use crate::gameplay::unit::view::*;
use movement::*;
use stats::*;

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
            .add_plugins(StatsPlugin)

            .add_event::<SpawnUnit>()

            .add_systems(OnEnter(AppState::Gameplay), test_require_spawn_unit)
            .add_systems(Update, spawn_unit.run_if(in_state(AppState::Gameplay)).in_set(Order::GameLogic))

            .add_systems(Update, test_target_position.after(spawn_unit))
        ;
    }
}

fn test_target_position(
    mut commands: Commands,

    units: Query<(Entity), Without<TargetPosition>>,
) {
    for unit in units.iter() {
        commands.entity(unit)
            .insert(TargetPosition(Vec2::new(100.0, 100.0)))
        ;
    }
}