use bevy::utils::HashMap;
use std::default::Default;
pub use modifier::*;
use crate::prelude::Reflect;

mod modifier;

#[derive(Reflect, Debug, Clone, Eq, PartialEq, Hash)]
pub enum Stat {
    MovementSpeed,
    Range,
    AttackChargeDuration,
    Attack,
    Defense,
    MaxHealth,
    SparkleCapacity,
    SparkleChargeRate,
}

pub struct StatProps<T> {
    pub movement_speed: T,
    pub range: T,
    pub attack_charge_duration: T,
    pub attack: T,
    pub defense: T,
    pub max_health: T,
    pub sparkle_capacity: T,
    pub sparkle_charge_rate: T,
}

impl<T: Default> Default for StatProps<T> {
    fn default() -> Self {
        StatProps {
            movement_speed: T::default(),
            range: T::default(),
            attack_charge_duration: T::default(),
            attack: T::default(),
            defense: T::default(),
            max_health: T::default(),
            sparkle_capacity: T::default(),
            sparkle_charge_rate: T::default(),
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
        stats.insert(Stat::Range, props.range);
        stats.insert(Stat::AttackChargeDuration, props.attack_charge_duration);
        stats.insert(Stat::Attack, props.attack);
        stats.insert(Stat::Defense, props.defense);
        stats.insert(Stat::MaxHealth, props.max_health);
        stats.insert(Stat::SparkleCapacity, props.sparkle_capacity);
        stats.insert(Stat::SparkleChargeRate, props.sparkle_charge_rate);

        StatsMap(stats)
    }
}