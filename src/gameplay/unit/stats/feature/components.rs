use crate::gameplay::unit::stats::{Modifier, Stat, StatProps, StatsMap};
use crate::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BaseStats(StatsMap<f32>);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct StatsModifiers(StatsMap<Modifier>);

impl BaseStats {
    pub fn new(props: StatProps<f32>) -> Self {
        Self(StatsMap::new(props))
    }

    pub fn get(&self, stat: &Stat) -> f32 {
        *self.0.0.get(stat)
            .unwrap_or_else(|| panic!("{stat:?} stat is missing!"))
    }
}

impl StatsModifiers {
    pub fn empty() -> Self {
        Self(StatsMap::empty())
    }

    pub fn get(&self, stat: &Stat) -> Modifier {
        *self.0.0.get(stat)
            .unwrap_or_else(|| panic!("{stat:?} stat is missing!"))
    }
}