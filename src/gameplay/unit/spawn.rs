use crate::common::CircleCollider;
use crate::gameplay::unit::lead::Lead;
use crate::gameplay::unit::stats::{BaseStats, StatProps, StatsModifiers};
use crate::gameplay::unit::view::LoadingView;
use crate::prelude::*;
use crate::prelude::status::*;

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
            .insert(Health(health))
            .insert(AutoAttackInRange)
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
            attack_range: 150.0,
            attack_charge_duration: 1.0,
            attack_damage: 5.0,
            max_health: 20.0,
        },
        UnitID::Rat => StatProps {
            movement_speed: 150.0,
            attack_range: 75.0,
            attack_charge_duration: 1.5,
            attack_damage: 2.0,
            max_health: 5.0,
        },
    }
}