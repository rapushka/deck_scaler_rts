use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use bevy::window::PrimaryWindow;
use crate::prelude::*;

pub use feature::*;
use crate::gameplay::unit::view::*;
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
    units: Query<Entity>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    for unit in units.iter() {
        for (camera, camera_transform) in cameras.iter() {
            for window in windows.iter() {
                let Some(cursor_position) = window.cursor_position() else {
                    continue;
                };

                let Ok(world_position) = camera.viewport_to_world(camera_transform, cursor_position) else {
                    continue;
                };

                let position = world_position.origin.truncate();

                commands.entity(unit)
                    .insert(TargetPosition(position))
                ;
            }
        }
    }
}