use crate::prelude::*;
pub use feature::*;
pub use types::*;
use crate::gameplay::affect::deal_damage::apply_deal_damage_affect;

mod feature;
mod types;

pub mod deal_damage;

pub struct AffectPlugin;

impl Plugin for AffectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                apply_deal_damage_affect,
            ).in_set(Order::ApplyAffects))

            .add_systems(Update, destroy_affects.after(Order::ApplyAffects))
        ;
    }
}