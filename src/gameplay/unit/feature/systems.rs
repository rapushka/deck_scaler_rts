use crate::gameplay::unit::behaviour_state::auto_mode::AutoAttackState;
use crate::gameplay::unit::health::Health;
use crate::gameplay::unit::lead::Lead;
use crate::gameplay::unit::side::feature::Side;
use crate::gameplay::unit::stats::*;
use crate::gameplay::unit::view::LoadingView;
use crate::input::*;
use crate::prelude::*;

pub fn test_require_spawn_unit(
    mut events: EventWriter<SpawnUnit>,
) {
    events.send(SpawnUnit {
        id: UnitID::Crook,
        position: Vec2::new(0.0, -200.0),
        side: Side::Player,
        is_lead: true,
    });

    events.send(SpawnUnit {
        id: UnitID::Crook,
        position: Vec2::new(100.0, -200.0),
        side: Side::Player,
        is_lead: false,
    });
}

pub fn spawn_unit(
    mut commands: Commands,
    mut events: EventReader<SpawnUnit>,
) {
    for SpawnUnit { id, position, side, is_lead } in events.read() {
        let id = *id;
        let stat_props = get_base_stats(id);
        let health = stat_props.max_health;

        let mut command = commands.spawn(Name::from(f!("{id:?}")));
        let unit = command
            .insert(id)
            .insert(LoadingView)
            .insert(BaseStats::new(stat_props))
            .insert(StatsModifiers::empty())
            .insert(CircleCollider::new(75.0))
            .insert(Transform::from_translation(position.extend(0.0)))
            .insert(*side)
            .insert(AutoAttackState)
            .insert(Health(health))
            .insert(Sparkle(1.0))
            .insert(NextSparkleCharge(0.0))
            ;

        if *is_lead {
            unit.insert(Lead);
        }
    }
}

fn get_base_stats(unit_id: UnitID) -> StatProps<f32> {
    match unit_id {
        UnitID::Crook => StatProps {
            movement_speed: 100.0,
            range: 150.0,
            attack_charge_duration: 1.0,
            attack: 5.0,
            defense: 1.0,
            max_health: 20.0,
            sparkle_capacity: 5.0,
            sparkle_charge_rate: 1.0,
        },
        UnitID::Rat => StatProps {
            movement_speed: 150.0,
            range: 75.0,
            attack_charge_duration: 1.5,
            attack: 2.0,
            defense: 1.0,
            max_health: 5.0,
            sparkle_capacity: 10.0,
            sparkle_charge_rate: 1.0,
        },
    }
}