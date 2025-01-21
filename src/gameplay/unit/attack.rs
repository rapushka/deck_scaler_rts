use crate::prelude::*;

pub use feature::*;

mod feature;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                start_attack_charging,
                tick_charging_attacks,
                on_attack_charged,
            ).chain()
                .in_set(Order::GameLogic))
        ;
    }
}