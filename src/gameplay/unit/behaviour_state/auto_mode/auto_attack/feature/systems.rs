use crate::gameplay::unit::behaviour_state::auto_mode::auto_attack::feature::AutoAttackState;
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

        let mut candidate: Option<(Entity, f32)> = None;

        for target in targets.iter() {
            let can_oppose = opponent_side == *target.2;
            if !can_oppose {
                continue;
            }

            let target_position = target.1.translation;
            let distance_to_target = attacker_position.distance(target_position);

            if candidate.is_none_or(|(_, d)| distance_to_target < d) {
                candidate = Some((target.0, distance_to_target));
            }
        }

        let mut entity = commands.entity(attacker.0);
        match candidate {
            Some((target, _)) => entity.insert(Opponent(target)),
            None => entity.remove::<Opponent>(),
        };
    }
}