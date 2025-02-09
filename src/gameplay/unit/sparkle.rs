use crate::prelude::*;

pub use feature::*;

mod feature;

pub struct SparklePlugin;

impl Plugin for SparklePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Sparkle>()
            .register_type::<SparkleCapacity>()
            .register_type::<NextSparkleCharge>()
            .register_type::<SparkleChargeRate>()

            .add_systems(Update, (
                multiply_stats_by_sparkle
            ).before(Order::UpdateStats))

            .add_systems(Update, (
                charge_sparkle,
                on_sparkle_charged,
            ).chain()
                .in_set(Order::GameLogic))
        ;
    }
}