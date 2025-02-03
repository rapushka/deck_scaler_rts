use crate::prelude::*;
use bevy::asset::AssetMetaCheck;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct ThirdPartyPlugin;

impl Plugin for ThirdPartyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins.set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }))
            .add_plugins(WorldInspectorPlugin::default())
            .add_plugins(DefaultTweenPlugins)
        ;
    }
}
