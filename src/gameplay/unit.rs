use crate::assets::UnitAssets;
use crate::prelude::*;

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
        .insert(Sprite {
            image: assets.crook.clone(),
            ..default()
        })
    ;
}