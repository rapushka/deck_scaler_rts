use crate::prelude::*;

pub mod app_state;
pub mod order;

pub struct InfrastructurePlugin;

impl Plugin for InfrastructurePlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_sets(Update, (
                Order::Input,
                Order::UnitOrders,
                Order::Preprocess,
                Order::GameLogic,
                Order::Postprocess,
                Order::View,
                Order::Cleanups,
            ).chain())

            .init_state::<AppState>()
        ;
    }
}