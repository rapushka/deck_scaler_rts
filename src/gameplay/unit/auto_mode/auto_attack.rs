use crate::gameplay::unit::auto_mode::auto_attack::feature::set_closest_enemy_as_opponent;
use crate::prelude::*;

mod feature;

pub struct AutoAttackPlugin;

impl Plugin for AutoAttackPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, set_closest_enemy_as_opponent.in_set(Order::GameLogic))
        ;
    }
}