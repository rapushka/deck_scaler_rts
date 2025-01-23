use crate::prelude::*;
pub use feature::*;
use crate::gameplay::unit::health::view::HealthViewPlugin;

mod feature;

mod view;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(HealthViewPlugin)
        ;
    }
}