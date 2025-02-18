use self::feature::*;
use crate::prelude::*;

mod feature {
    pub(super) use systems::*;
    pub use commands::*;

    mod systems;
    mod commands;
}

pub struct UnitOrdersViewPlugin;

impl Plugin for UnitOrdersViewPlugin {
    fn build(&self, app: &mut App) {
        app

            .add_systems(Update, (
                view_target_position_order,
                view_target_unit_order,
            )
                .run_if(in_state(AppState::Gameplay))
                .after(Order::UnitOrders))
        ;
    }
}

