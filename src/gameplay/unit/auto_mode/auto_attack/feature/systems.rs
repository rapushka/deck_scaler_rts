use crate::gameplay::unit::auto_mode::auto_attack::feature::AutoAttackState;
use crate::gameplay::unit::opponent::Opponent;
use crate::gameplay::unit::side::feature::Side;
use crate::prelude::*;

pub fn set_closest_enemy_as_opponent(
    mut commands: Commands,
    attackers: Query<(Entity, &Transform, &Side), (With<UnitID>, With<AutoAttackState>)>,
    targets: Query<(Entity, &Transform, &Side), With<UnitID>>,
) {
    for attacker in attackers.iter() {
        let opponent_side = attacker.2.flip();
        let attacker_position = attacker.1.translation;

        let mut cashed_enemy = None;
        let mut cashed_distance = None;

        for target in targets.iter() {
            let can_be_opponent = opponent_side == *target.2;
            if !can_be_opponent {
                continue;
            }

            let target_position = target.1.translation;
            let distance_to_target = attacker_position.distance(target_position);

            if cashed_distance.is_some_and(|d| d >= distance_to_target) {
                continue;
            }

            cashed_enemy = Some(target.0);
            cashed_distance = Some(distance_to_target);
        }

        let mut entity = commands.entity(attacker.0);
        match cashed_enemy {
            Some(target) => entity.insert(Opponent(target)),
            None => entity.remove::<Opponent>(),
        };
    }
}