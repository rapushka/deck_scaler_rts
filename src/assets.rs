use crate::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
                LoadingState::new(AppState::Bootstrap)
                    .load_collection::<UnitAssets>()
                    .continue_to_state(AppState::Gameplay)
            )
        ;
    }
}

#[derive(AssetCollection, Resource)]
pub struct UnitAssets {
    #[asset(path = "unit/crook_head_small.png")]
    pub crook: Handle<Image>,
}
