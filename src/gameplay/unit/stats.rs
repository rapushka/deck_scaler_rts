use crate::prelude::*;
pub use feature::*;
pub use structs::*;

mod feature;

mod structs;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_movement_speed,
            ))
        ;
    }
}