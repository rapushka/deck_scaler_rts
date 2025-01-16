use crate::gameplay::unit::view::feature::*;
use crate::prelude::*;

pub use feature::*;
mod feature;

pub struct UnitViewPlugin;

impl Plugin for UnitViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, load_unit_view.run_if(in_state(AppState::Gameplay)))
        ;
    }
}