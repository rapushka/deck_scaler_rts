use crate::third_party::ThirdPartyPlugin;

use bevy::prelude::*;

mod prelude;
mod third_party;

fn main() {
    App::new()
        .add_plugins(ThirdPartyPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite {
        image: asset_server.load("ducky.png"),
        ..Default::default()
    });
}
