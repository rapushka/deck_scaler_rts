use crate::gameplay::unit::stats::{Modifier, Stat, StatsModifiers};
use crate::prelude::*;

pub fn multiply_stats_by_sparkle(
    mut entities: Query<(&mut StatsModifiers, &Sparkle, &Suit)>,
) {
    for (mut modifiers, Sparkle(sparkle), suit) in entities.iter_mut() {
        let target_stat = match suit {
            Suit::Spades => Stat::Attack,
            Suit::Clubs => Stat::Defense,
            Suit::Hearts => Stat::Attack, // TODO:
            Suit::Diamonds => Stat::Attack, // TODO:
        };

        modifiers.set(target_stat, Modifier::multiply(*sparkle));
    }
}

pub fn charge_sparkle(
    mut entities: Query<(&mut NextSparkleCharge, &SparkleChargeRate)>,
    time: Res<Time<Virtual>>,
) {
    for (mut next_sparkle, charge_rate) in entities.iter_mut() {
        next_sparkle.0 += charge_rate.0 * time.delta_secs();
    }
}

pub fn on_sparkle_charged(
    mut entities: Query<(&mut NextSparkleCharge, &SparkleCapacity, &mut Sparkle)>,
) {
    for (mut next_charge, capacity, mut sparkle) in entities.iter_mut() {
        if next_charge.0 < capacity.0 {
            continue;
        }

        next_charge.0 = 0.0;
        sparkle.0 += 1.0;
    }
}