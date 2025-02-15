use crate::gameplay::level::feature::spawn_test_level;
use crate::prelude::*;

mod feature;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameplay), spawn_test_level)
        ;
    }
}