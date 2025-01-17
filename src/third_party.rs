use crate::prelude::*;
use bevy::asset::AssetMetaCheck;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct ThirdPartyPlugin;

impl Plugin for ThirdPartyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins.set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics in web builds on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            }))
            .add_plugins(WorldInspectorPlugin::default())
        ;
    }
}
