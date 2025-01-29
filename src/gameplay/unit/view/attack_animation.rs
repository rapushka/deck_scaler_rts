use feature::*;
use crate::prelude::*;

mod feature;

pub(super) struct UnitAttackAnimationPlugin;

impl Plugin for UnitAttackAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, play_attack_animation
                .run_if(in_state(AppState::Gameplay))
                .in_set(Order::View),
            )
        ;
    }
}