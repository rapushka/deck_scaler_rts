use crate::gameplay::unit::attack::*;
use crate::gameplay::unit::stats::*;

pub fn update_stats(
    mut commands: Commands,
    entities: Query<(Entity, &BaseStats, &StatsModifiers)>,
) {
    for (entity, base_stats, modifiers) in entities.iter() {
        let calculate = |stat: Stat| -> f32 {
            modifiers.get(&stat).modify(base_stats.get(&stat))
        };

        commands.entity(entity)
            .insert(MovementSpeed(calculate(Stat::MovementSpeed)))
            .insert(Range(calculate(Stat::Range)))
            .insert(AttackChargeDuration(calculate(Stat::AttackChargeDuration)))
            .insert(Attack(calculate(Stat::Attack)))
            .insert(Defense(calculate(Stat::Defense)))
            .insert(MaxHealth(calculate(Stat::MaxHealth)))
            .insert(SparkleCapacity(calculate(Stat::SparkleCapacity)))
            .insert(SparkleChargeRate(calculate(Stat::SparkleChargeRate)))
        ;
    }
}
