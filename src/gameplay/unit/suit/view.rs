use crate::prelude::*;
pub use feature::*;

mod feature;

pub struct SuitViewPlugin;

impl Plugin for SuitViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<SuitView>()

            .add_systems(Update, (
                spawn_suit_view,
                update_suit_view,
            ).chain()
                .run_if(in_state(AppState::Gameplay))
                .in_set(Order::View))
        ;
    }
}