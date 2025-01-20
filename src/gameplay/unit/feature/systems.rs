use crate::gameplay::unit::stats::*;
use crate::gameplay::unit::view::LoadingView;
use crate::input::{CursorPosition, JustClicked, PlayerInput};
use crate::prelude::selection::feature::SelectedUnit;
use crate::prelude::*;

pub fn test_require_spawn_unit(
    mut events: EventWriter<SpawnUnit>,
) {
    events.send(SpawnUnit(UnitID::Crook));
}

pub fn spawn_unit(
    mut commands: Commands,
    mut events: EventReader<SpawnUnit>,
) {
    for spawn_unit in events.read() {
        let unit_id = spawn_unit.0.clone();

        let stats = Stats::from(StatProps {
            movement_speed: 100.0,
        });
        commands.spawn(Name::from(f!("{unit_id:?}")))
            .insert(unit_id)
            .insert(LoadingView)
            .insert(BaseStats(stats))
            .insert(StatsModifiers(Stats::empty()))
            .insert(CircleCollider::new(100.0))
        ;
    }
}

pub fn test_target_position(
    mut commands: Commands,
    units: Query<Entity, (With<UnitID>, With<SelectedUnit>)>,
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