use crate::prelude::*;
pub use feature::*;
pub use types::*;

mod feature;
mod types;

pub mod deal_damage;

pub struct AffectPlugin;

impl Plugin for AffectPlugin {
    fn build(&self, app: &mut App) {
        app
        ;
    }
}