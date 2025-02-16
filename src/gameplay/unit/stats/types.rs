use bevy::utils::HashMap;
use std::default::Default;
pub use modifier::*;
use crate::common::MovementSpeed;
use crate::gameplay::unit::attack::{AttackDamage, AttackChargeDuration, Range};
use crate::gameplay::unit::stats::{BaseStats, StatsModifiers};
use crate::prelude::*;

mod modifier;

#[derive(Reflect, Debug, Clone, Eq, PartialEq, Hash)]
pub enum Stat {
    MovementSpeed,
    AttackRange,
    AttackChargeDuration,
    AttackDamage,
    MaxHealth,
}

pub struct StatProps<T> {
    pub movement_speed: T,
    pub attack_range: T,
    pub attack_charge_duration: T,
    pub attack_damage: T,
    pub max_health: T,
}

impl<T: Default> Default for StatProps<T> {
    fn default() -> Self {
        StatProps {
            movement_speed: T::default(),
            attack_range: T::default(),
            attack_charge_duration: T::default(),
            attack_damage: T::default(),
            max_health: T::default(),
        }
    }
}

#[derive(Reflect)]
pub struct StatsMap<T>(pub HashMap<Stat, T>);

impl<T: Default> StatsMap<T> {
    pub fn empty() -> Self {
        Self::new(StatProps::default())
    }

    pub fn new(props: StatProps<T>) -> Self {
        let mut stats = HashMap::new();
        stats.insert(Stat::MovementSpeed, props.movement_speed);
        stats.insert(Stat::AttackRange, props.attack_range);
        stats.insert(Stat::AttackChargeDuration, props.attack_charge_duration);
        stats.insert(Stat::AttackDamage, props.attack_damage);
        stats.insert(Stat::MaxHealth, props.max_health);

        StatsMap(stats)
    }
}

pub fn read_stats(mut entity: EntityCommands, base_stats: &BaseStats, modifiers: &StatsModifiers) {
    let calculate = |stat: Stat| -> f32 {
        modifiers.get(&stat).modify(base_stats.get(&stat))
    };

    entity
        .insert(MovementSpeed(calculate(Stat::MovementSpeed)))
        .insert(Range(calculate(Stat::AttackRange)))
        .insert(AttackChargeDuration(calculate(Stat::AttackChargeDuration)))
        .insert(AttackDamage(calculate(Stat::AttackDamage)))
        .insert(MaxHealth(calculate(Stat::MaxHealth)))
    ;
}