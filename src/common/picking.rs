use crate::prelude::*;

pub use feature::*;

mod feature;

pub struct PickingPlugin;

impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ClickSelect>()
            .add_event::<ClickTargetPosition>()
            .add_event::<ClickTargetUnit>()

            .add_systems(Update, (
                handle_select_clicks,
                handle_target_clicks,
            ).in_set(Order::Input))
        ;
    }
}