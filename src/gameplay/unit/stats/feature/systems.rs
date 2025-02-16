use crate::gameplay::unit::stats::{read_stats, BaseStats, StatsModifiers};
use crate::prelude::{Commands, Entity, Query};

pub fn update_stats(
    mut commands: Commands,
    entities: Query<(Entity, &BaseStats, &StatsModifiers)>,
) {
    for (entity, base_stats, modifiers) in entities.iter() {
        read_stats(commands.entity(entity), base_stats, modifiers);
    }
}