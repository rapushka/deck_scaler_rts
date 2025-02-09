use crate::prelude::*;

pub use feature::*;

mod feature;

pub struct SparklePlugin;

impl Plugin for SparklePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Sparkle>()
            .register_type::<SparkleChargeDuration>()

            .add_systems(Update, (
                multiply_stats_by_sparkle
            ).before(Order::UpdateStats))
        ;
    }
}