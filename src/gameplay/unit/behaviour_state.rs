use crate::gameplay::unit::behaviour_state::auto_mode::AutoAttackPlugin;
use crate::prelude::*;
use state_change::*;

pub mod auto_mode;
mod state_change;

pub struct UnitBehaviourPlugin;

impl Plugin for UnitBehaviourPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UnitStateChangePlugin)
            .add_plugins(AutoAttackPlugin)
        ;
    }
}