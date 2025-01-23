use crate::prelude::*;
pub use feature::*;
use crate::gameplay::unit::feature::spawn_unit;

mod feature;

pub struct HealthViewPlugin;

impl Plugin for HealthViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<HealthBar>()

            .add_systems(Update, spawn_health_bar.after(spawn_unit))
        ;
    }
}