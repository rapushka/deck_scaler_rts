use crate::prelude::*;
pub use feature::*;
use crate::gameplay::unit::feature::spawn_unit;

mod feature;

pub struct LeadPlugin;

impl Plugin for LeadPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, select_lead_on_start
                .run_if(in_state(AppState::Gameplay))
                .after(Order::GameLogic),
            )
        ;
    }
}