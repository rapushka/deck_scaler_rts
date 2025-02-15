use crate::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
                LoadingState::new(AppState::Bootstrap)
                    .load_collection::<UnitAssets>()
                    .load_collection::<SuitAssets>()
                    .load_collection::<UiAssets>()
                    .continue_to_state(AppState::Gameplay)
            )
        ;
    }
}

#[derive(AssetCollection, Resource)]
pub struct UnitAssets {
    #[asset(path = "unit/crook_head_small.png")]
    pub crook: Handle<Image>,

    #[asset(path = "unit/rat_head_small.png")]
    pub rat: Handle<Image>,
}

impl UnitAssets {
    pub fn get_head(&self, unit_id: &UnitID) -> Handle<Image> {
        match unit_id {
            UnitID::Crook => self.crook.clone(),
            UnitID::Rat => self.rat.clone(),
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct SuitAssets {
    #[asset(path = "suits/clubs.png")]
    pub clubs: Handle<Image>,

    #[asset(path = "suits/diamonds.png")]
    pub diamonds: Handle<Image>,

    #[asset(path = "suits/hearts.png")]
    pub hearts: Handle<Image>,

    #[asset(path = "suits/spades.png")]
    pub spades: Handle<Image>,
}

impl SuitAssets {
    pub fn get_sprite(&self, suit: &Suit) -> Handle<Image> {
        match suit {
            Suit::Spades => self.spades.clone(),
            Suit::Hearts => self.hearts.clone(),
            Suit::Clubs => self.clubs.clone(),
            Suit::Diamonds => self.diamonds.clone(),
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "ui/arrows.png")]
    pub arrows: Handle<Image>,
}