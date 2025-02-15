use crate::prelude::*;
pub use feature::*;

mod feature;

pub struct LeadPlugin;

impl Plugin for LeadPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, select_lead_on_start
                .run_if(not(any_with_component::<SelectedUnit>))
                .after(Order::SelectUnits),
            )
        ;
    }
}