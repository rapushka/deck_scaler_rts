use std::time::Duration;
use crate::gameplay::affect::{Affect, AffectType, CreateAffect};
use crate::gameplay::unit::attack::{AttackCharged, AttackChargeDuration, ChargingAttack, Range, Damage};
use crate::gameplay::unit::opponent::Opponent;
use crate::prelude::*;

pub fn start_attack_charging(
    mut commands: Commands,
    attackers: Query<(Entity, &Opponent, &Range, &AttackChargeDuration), (With<UnitID>, Without<ChargingAttack>)>,
    transforms: Query<&Transform>,
) {
    for (attacker, opponent, range, charge_duration) in attackers.iter() {
        let attacker_position = transforms.get(attacker).unwrap().translation.truncate();
        let opponent_position = transforms.get(opponent.0).unwrap().translation.truncate();

        let distance = attacker_position.distance(opponent_position);
        if distance > range.0 {
            continue;
        }

        let charge_duration = Duration::from_secs_f32(charge_duration.0);

        commands.entity(attacker)
            .insert(ChargingAttack(Timer::new(charge_duration, TimerMode::Once)))
        ;
    }
}

pub fn tick_charging_attacks(
    mut attackers: Query<&mut ChargingAttack>,
    time: Res<Time>,
) {
    for mut timer in attackers.iter_mut() {
        timer.0.tick(time.delta());
    }
}

pub fn on_attack_charged(
    mut commands: Commands,
    attackers: Query<(Entity, &ChargingAttack)>,
    mut events: EventWriter<AttackCharged>,
) {
    for (attacker, timer) in attackers.iter() {
        if !timer.0.finished() {
            continue;
        }

        events.send(AttackCharged(attacker));

        commands.entity(attacker)
            .remove::<ChargingAttack>()
        ;
    }
}

pub fn create_attack_affect(
    mut commands: Commands,
    mut events: EventReader<AttackCharged>,
    attackers: Query<(Entity, &Opponent, &Damage,)>,
) {
    for AttackCharged(attacker) in events.read() {
        let (attacker, opponent, damage) = attackers.get(*attacker)
            .expect("Entity missed necessary components");

        commands.queue(CreateAffect {
            affect_type: AffectType::DealDamage,
            value: damage.0,
            sender: attacker,
            target: opponent.0,
        });
    }
}