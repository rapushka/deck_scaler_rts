use crate::prelude::*;

pub use feature::*;

mod feature;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, load_sprite)
        ;
    }
}