use crate::prelude::*;
pub use feature::*;

mod feature;

pub struct AutoAttackPlugin;

impl Plugin for AutoAttackPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, set_closest_enemy_as_opponent.in_set(Order::GameLogic))
        ;
    }
}