use crate::prelude::*;

pub use feature::*;

mod feature;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<AttackCharged>()

            .add_systems(Update, (
                start_attack_charging,
                tick_charging_attacks,
                on_attack_charged,
            ).chain()
                .in_set(Order::ChargeAttack))

            .add_systems(Update, create_attack_affect.in_set(Order::CreateAffects))
        ;
    }
}