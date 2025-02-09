use crate::gameplay::unit::attack::*;
use crate::gameplay::unit::health::*;
use crate::gameplay::unit::stats::*;
use crate::prelude::*;

pub fn update_stats(
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
            .insert(AttackChargeDuration(calculate(Stat::AttackChargeDuration)))
            .insert(MaxHealth(calculate(Stat::MaxHealth)))
            .insert(Damage(calculate(Stat::Attack)))
        ;
    }
}
