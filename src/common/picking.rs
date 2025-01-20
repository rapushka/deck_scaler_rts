use crate::prelude::*;

pub use feature::*;

mod feature;

pub struct PickingPlugin;

impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Clicked>()

            .add_systems(Update, handle_clicks.in_set(Order::Input))
        ;
    }
}