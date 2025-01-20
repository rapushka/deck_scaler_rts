use crate::gameplay::unit::side::feature::Side;
use crate::gameplay::unit::stats::*;
use crate::gameplay::unit::view::LoadingView;
use crate::input::{CursorPosition, JustClicked, PlayerInput};
use crate::prelude::selection::feature::SelectedUnit;
use crate::prelude::*;

pub fn test_require_spawn_unit(
    mut events: EventWriter<SpawnUnit>,
) {
    events.send(SpawnUnit {
        id: UnitID::Crook,
        position: Vec2::new(0.0, 0.0),
        side: Side::Player,
    });

    events.send(SpawnUnit {
        id: UnitID::Crook, // TODO: add new unit id for enemy
        position: Vec2::new(200.0, 0.0),
        side: Side::Enemy,
    });
}

pub fn spawn_unit(
    mut commands: Commands,
    mut events: EventReader<SpawnUnit>,
) {
    for SpawnUnit { id, position, side } in events.read() {
        let stats = Stats::from(StatProps {
            movement_speed: 100.0,
        });

        commands.spawn(Name::from(f!("{id:?}")))
            .insert(*id)
            .insert(LoadingView)
            .insert(BaseStats(stats))
            .insert(StatsModifiers(Stats::empty()))
            .insert(CircleCollider::new(100.0))
            .insert(Transform::from_translation(position.extend(0.0)))
            .insert(*side)
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