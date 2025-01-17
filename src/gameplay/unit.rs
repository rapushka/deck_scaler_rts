use crate::prelude::*;

use crate::gameplay::unit::view::*;
use crate::input::{CursorPosition, JustClicked, PlayerInput};
pub use feature::*;
use stats::*;

mod feature;
mod view;
mod stats;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UnitViewPlugin)
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
    units: Query<Entity, With<UnitID>>,
    cursors: Query<&CursorPosition, (With<PlayerInput>, With<JustClicked>)>,
) {
    for unit in units.iter() {
        for cursor_position in cursors.iter() {
            commands.entity(unit)
                .insert(TargetPosition(cursor_position.0))
            ;
        }
    }
}
