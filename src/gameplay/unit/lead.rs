use crate::prelude::*;
pub use feature::*;

mod feature;

pub struct LeadPlugin;

impl Plugin for LeadPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, move_lead.in_set(Order::MoveUnits))
        ;
    }
}