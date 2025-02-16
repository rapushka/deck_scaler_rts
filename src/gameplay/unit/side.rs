use crate::gameplay::unit::side::feature::on_side_added;
use crate::gameplay::unit::spawn::spawn_unit;
use crate::prelude::*;

pub mod feature;

pub struct UnitSidePlugin;

impl Plugin for UnitSidePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, on_side_added.after(spawn_unit))
        ;
    }
}