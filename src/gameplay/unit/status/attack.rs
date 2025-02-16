use crate::prelude::*;
use crate::prelude::status::AutoAttackInRange;

pub struct UnitAttackStatusPlugin;

impl Plugin for UnitAttackStatusPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, auto_attack
                .in_set(Order::GameLogic))
        ;
    }
}

fn auto_attack(
    units: Query<Entity, With<AutoAttackInRange>>
) {}