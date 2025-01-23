use crate::gameplay::unit::behaviour_state::auto_mode::AutoAttackPlugin;
use crate::prelude::*;

pub mod auto_mode;

pub struct UnitBehaviourPlugin;

impl Plugin for UnitBehaviourPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(AutoAttackPlugin)
        ;
    }
}