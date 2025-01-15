use crate::prelude::*;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, test_spawn_sprite);
    }
}

fn test_spawn_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Sprite {
        image: asset_server.load("ducky.png"),
        ..Default::default()
    });
}
