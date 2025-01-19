use crate::prelude::*;
pub use feature::*;
pub use structs::*;

mod feature;

mod structs;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<BaseStats>()
            .register_type::<StatsModifiers>()

            .add_systems(Update, (
                update_movement_speed,
            ).in_set(Order::Preprocess))
        ;
    }
}