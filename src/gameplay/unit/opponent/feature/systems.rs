use crate::gameplay::unit::attack::Range;
use crate::gameplay::unit::opponent::Opponent;
use crate::prelude::*;

pub fn move_to_opponent(
    mut commands: Commands,
    attackers: Query<(Entity, &Opponent, &Range), With<UnitID>>,
    transforms: Query<&Transform>,
) {
    for (attacker, opponent, range) in attackers.iter() {
        let attacker_position = cq!(transforms.get(attacker)).translation.truncate();
        let opponent_position = cq!(transforms.get(opponent.0)).translation.truncate();

        let distance = attacker_position.distance(opponent_position);
        if distance < range.0 {
            commands.entity(attacker).remove::<TargetPosition>();
            continue;
        }

        commands.entity(attacker).insert(TargetPosition(opponent_position));
    }
}