use crate::gameplay::unit::opponent::Opponent;
use crate::gameplay::unit::side::feature::*;
use super::components::*;
use crate::prelude::*;
use crate::prelude::status::AutoAttackInRange;

pub fn find_opponent_in_range(
    mut commands: Commands,
    units: Query<(Entity, &Transform, &TriggerDistance, &OnSide), (With<UnitID>, With<AutoAttackInRange>)>,
    potential_targets: Query<(Entity, &Transform, &OnSide), With<UnitID>>,
) {
    for (unit, transform, TriggerDistance(max_distance), side) in units.iter() {
        let opposite_side = side.flip();
        let unit_position = transform.translation;

        let mut closest_target = None;
        let mut closest_distance = None;

        for (target, target_transform, target_side) in potential_targets.iter() {
            if *target_side != opposite_side {
                continue;
            }

            let target_position = target_transform.translation;

            let new_distance = unit_position.distance(target_position);
            if new_distance > *max_distance {
                continue;
            }

            if closest_distance.is_none_or(|saved_distance| new_distance < saved_distance) {
                closest_distance = Some(new_distance);
                closest_target = Some(target);
            }
        }

        let mut unit = commands.entity(unit);
        match closest_target {
            Some(target) => unit.insert(Opponent(target)),
            None => unit.remove::<Opponent>(),
        };
    }
}