use crate::gameplay::unit::health::Health;
use crate::gameplay::unit::side::feature::Side;
use crate::gameplay::unit::stats::*;
use crate::gameplay::unit::view::LoadingView;
use crate::input::{CursorPosition, JustClickedOrder, PlayerInput};
use crate::prelude::selection::feature::SelectedUnit;
use crate::prelude::*;
use crate::gameplay::unit::behaviour_state::auto_mode::AutoAttackState;

pub fn test_require_spawn_unit(
    mut events: EventWriter<SpawnUnit>,
) {
    events.send(SpawnUnit {
        id: UnitID::Crook,
        position: Vec2::new(0.0, -200.0),
        side: Side::Player,
    });
    events.send(SpawnUnit {
        id: UnitID::Crook,
        position: Vec2::new(-150.0, -200.0),
        side: Side::Player,
    });

    events.send(SpawnUnit {
        id: UnitID::Rat,
        position: Vec2::new(100.0, 200.0),
        side: Side::Enemy,
    });
}

pub fn spawn_unit(
    mut commands: Commands,
    mut events: EventReader<SpawnUnit>,
) {
    for SpawnUnit { id, position, side } in events.read() {
        let id = *id;
        let stat_props = get_base_stats(id);
        let health = stat_props.max_health;

        commands.spawn(Name::from(f!("{id:?}")))
            .insert(id)
            .insert(LoadingView)
            .insert(BaseStats::new(stat_props))
            .insert(StatsModifiers::empty())
            .insert(CircleCollider::new(75.0))
            .insert(Transform::from_translation(position.extend(0.0)))
            .insert(*side)
            .insert(AutoAttackState)
            .insert(Health(health))
        ;
    }
}

fn get_base_stats(unit_id: UnitID) -> StatProps<f32> {
    match unit_id {
        UnitID::Crook => StatProps {
            movement_speed: 100.0,
            range: 150.0,
            attack_charge_duration: 1.0,
            max_health: 20.0,
        },
        UnitID::Rat => StatProps {
            movement_speed: 150.0,
            range: 75.0,
            attack_charge_duration: 1.5,
            max_health: 5.0,
        },
    }
}

pub fn order_target_position(
    mut commands: Commands,
    units: Query<Entity, (With<UnitID>, With<SelectedUnit>)>,
    cursors: Query<&CursorPosition, (With<PlayerInput>, With<JustClickedOrder>)>,
) {
    for unit in units.iter() {
        for cursor_position in cursors.iter() {
            commands.entity(unit)
                .insert(TargetPosition(cursor_position.0))
            ;
        }
    }
}