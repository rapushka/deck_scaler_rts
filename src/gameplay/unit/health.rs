use crate::prelude::*;
pub use feature::*;

mod feature;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app

        ;
    }
}