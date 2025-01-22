use crate::gameplay::unit::stats::{Modifier, Stat, StatsMap};
use crate::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BaseStats(pub StatsMap<f32>);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct StatsModifiers(pub StatsMap<Modifier>);

impl BaseStats {
    pub fn get(&self, stat: &Stat) -> f32 {
        *self.0.0.get(stat)
            .unwrap_or_else(|| panic!("{stat:?} stat is missing!"))
    }
}

impl StatsModifiers {
    pub fn get(&self, stat: &Stat) -> Modifier {
        *self.0.0.get(stat)
            .unwrap_or_else(|| panic!("{stat:?} stat is missing!"))
    }
}