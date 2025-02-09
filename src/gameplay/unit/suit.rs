use crate::prelude::*;
pub use feature::*;
use view::SuitViewPlugin;

mod feature;

pub mod view;

pub struct SuitsPlugin;

impl Plugin for SuitsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SuitViewPlugin)

            .register_type::<Suit>()
        ;
    }
}