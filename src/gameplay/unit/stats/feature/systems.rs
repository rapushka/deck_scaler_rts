use crate::gameplay::unit::stats::{BaseStats, MovementSpeed, Stat, StatsModifiers};
use crate::prelude::*;

pub fn update_movement_speed(
    mut commands: Commands,
    entities: Query<(Entity, &BaseStats, &StatsModifiers)>,
) {
    for (entity, base_stats, modifiers) in entities.iter() {
        let base_speed = base_stats.get(&Stat::MovementSpeed);
        let modifier = modifiers.get(&Stat::MovementSpeed);

        let speed = modifier.modify(base_speed);

        commands.entity(entity)
            .insert(MovementSpeed(speed))
        ;
    }
}