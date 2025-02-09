use crate::prelude::deal_damage::DealDamage;
use crate::prelude::*;

pub(super) fn block_damage(
    mut affects: Query<(&mut AffectValue, &Target), With<DealDamage>>,
    defense_stats: Query<&Defense>,
) {
    for (mut value, Target(target)) in affects.iter_mut() {
        let target_defense = cq!(defense_stats.get(*target));
        value.0 -= target_defense.0;
    }
}