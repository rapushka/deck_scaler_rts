use crate::gameplay::affect::deal_damage::*;
use crate::prelude::*;

pub fn apply_deal_damage_affect(
    affects: Query<(&AffectValue, &Target), With<DealDamage>>,
    mut healths: Query<&mut Health>,
) {
    for (value, target) in affects.iter() {
        let mut target_health = cq!(healths.get_mut(target.0));
        let damage = value.0.clamp_min(0.0);

        target_health.0 -= damage;
    }
}