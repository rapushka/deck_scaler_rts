use crate::gameplay::unit::stats::{BaseStats, Modifier, Stat, StatsModifiers};
use crate::prelude::*;
use crate::prelude::deal_damage::DealDamage;

pub fn multiply_stats_by_sparkle(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut StatsModifiers, &Sparkle, &Suit)>,
) {
    for (entity, mut modifiers, Sparkle(sparkle), suit) in entities.iter_mut() {
        let target_stat = match suit {
            Suit::Spades => Stat::Attack,
            Suit::Clubs => Stat::Defense,
            Suit::Hearts => todo!(),
            Suit::Diamonds => todo!(),
        };

        modifiers.set(target_stat, Modifier::multiply(*sparkle));
    }
}