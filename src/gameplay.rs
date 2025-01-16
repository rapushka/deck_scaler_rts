use crate::gameplay::unit::
UnitPlugin;
use crate::prelude::*;

pub mod unit;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UnitPlugin)
        ;
    }
}