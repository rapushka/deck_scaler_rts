use crate::prelude::*;
pub use feature::*;

mod feature;

pub struct SuitsPlugin;

impl Plugin for SuitsPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Suit>()
        ;
    }
}