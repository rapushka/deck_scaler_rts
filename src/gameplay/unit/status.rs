use crate::prelude::*;
pub use types::*;
use crate::prelude::status::attack::UnitAttackStatusPlugin;

mod types;

mod attack;

pub struct UnitStatusPlugin;

impl Plugin for UnitStatusPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UnitAttackStatusPlugin)
        ;
    }
}