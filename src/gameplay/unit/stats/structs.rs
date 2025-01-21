use bevy::utils::HashMap;
use std::default::Default;

pub use modifier::*;
use crate::prelude::Reflect;

mod modifier;

pub struct StatProps {
    pub movement_speed: f32,
    pub range: f32,
    pub attack_charge_duration: f32,
}

#[derive(Reflect, Debug, Clone, Eq, PartialEq, Hash)]
pub enum Stat {
    MovementSpeed,
    Range,
    AttackChargeDuration,
}

#[derive(Reflect)]
pub struct Stats<T>(pub HashMap<Stat, T>);

impl<T: Default> Stats<T> {
    pub fn empty() -> Self {
        Self::new(
            T::default(),
            T::default(),
            T::default(),
        )
    }

    pub fn new(movement_speed: T, range: T, attack_charge_duration: T) -> Self {
        let mut stats = HashMap::new();
        stats.insert(Stat::MovementSpeed, movement_speed);
        stats.insert(Stat::Range, range);
        stats.insert(Stat::AttackChargeDuration, attack_charge_duration);

        Stats(stats)
    }
}

impl Stats<f32> {
    pub fn from(props: StatProps) -> Self {
        Self::new(
            props.movement_speed,
            props.range,
            props.attack_charge_duration,
        )
    }
}