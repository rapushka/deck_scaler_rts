use crate::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, toggle_pause.in_set(Order::Input))
        ;
    }
}

fn toggle_pause(
    input: Res<ButtonInput<KeyCode>>,
    mut time: ResMut<Time<Virtual>>,
) {
    if !input.just_pressed(KeyCode::KeyT) {
        return;
    }

    if !time.is_paused() {
        time.pause();
    } else {
        time.unpause();
    }
}