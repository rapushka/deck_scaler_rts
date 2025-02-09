use crate::prelude::*;
pub use feature::*;
pub use types::*;
use deal_damage::*;
use block_damage::*;

mod feature;
mod types;

pub mod deal_damage;
pub mod block_damage;

pub struct AffectPlugin;

impl Plugin for AffectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                block_damage,
            ).in_set(Order::InfluenceAffects))

            .add_systems(Update, (
                apply_deal_damage_affect,
            ).in_set(Order::ApplyAffects))

            .add_systems(Update, destroy_affects.after(Order::ApplyAffects))
        ;
    }
}