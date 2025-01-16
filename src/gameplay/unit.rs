use crate::assets::UnitAssets;
use crate::prelude::*;
use crate::view::RequireSprite;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameplay), test_spawn_unit)
        ;
    }
}

fn test_spawn_unit(
    mut commands: Commands,
    assets: Res<UnitAssets>,
) {
    commands.spawn(Name::from("crook"))
        .insert(RequireSprite(assets.crook.clone()))
    ;
}