use crate::prelude::*;
use feature::*;
use view::*;

mod feature;
mod view;

pub struct UnitOrdersPlugin;

impl Plugin for UnitOrdersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UnitOrdersViewPlugin)

            .add_systems(Update, order_target_position
                .run_if(in_state(AppState::Gameplay))
                .in_set(Order::UnitOrders),
            )
        ;
    }
}