use bevy::utils::HashMap;
use std::default::Default;

pub use modifier::*;

mod modifier;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Stat {
    MovementSpeed,
}

pub struct Stats<T>(pub HashMap<Stat, T>);

impl<T: Default> Stats<T> {
    pub fn empty() -> Self {
        Self::new(T::default())
    }

    pub fn new(movement_speed: T) -> Self {
        let mut stats = HashMap::new();
        stats.insert(Stat::MovementSpeed, movement_speed);

        Stats(stats)
    }
}


