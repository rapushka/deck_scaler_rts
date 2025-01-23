use crate::prelude::*;
pub use feature::*;

mod feature;

pub struct HealthViewPlugin;

impl Plugin for HealthViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<HealthBar>()

            .add_systems(Update, (
                spawn_health_bars,
                update_health_bars,
            ).in_set(Order::Postprocess))
        ;
    }
}