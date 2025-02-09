use crate::prelude::*;
pub use feature::*;

mod feature;

pub struct DefensePlugin;

impl Plugin for DefensePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, block_income_damage_by_defense.in_set(Order::InfluenceAffects))
        ;
    }
}