use crate::prelude::*;
pub use feature::*;
pub use types::*;

mod feature;

mod types;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<BaseStats>()
            .register_type::<StatsModifiers>()

            .add_systems(Update, (
                update_stats,
            ).in_set(Order::Preprocess))
        ;
    }
}