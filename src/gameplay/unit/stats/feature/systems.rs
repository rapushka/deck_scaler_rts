use crate::gameplay::unit::attack::Range;
use crate::gameplay::unit::stats::{BaseStats, MovementSpeed, Stat, StatsModifiers};
use crate::prelude::*;

pub fn update_movement_speed(
    mut commands: Commands,
    entities: Query<(Entity, &BaseStats, &StatsModifiers)>,
) {
    for (entity, base_stats, modifiers) in entities.iter() {
        let calculate = |stat: Stat| -> f32{
            modifiers.get(&stat).modify(base_stats.get(&stat))
        };

        commands.entity(entity)
            .insert(MovementSpeed(calculate(Stat::MovementSpeed)))
            .insert(Range(calculate(Stat::Range)))
        ;
    }
}