use crate::assets::UnitAssets;
use crate::prelude::*;
use crate::view::RequireSprite;

pub fn test_require_spawn_unit(
    mut commands: Commands,
) {
    commands.spawn(SpawnUnit(UnitID::Crook));
}

pub fn spawn_unit(
    mut commands: Commands,
    assets: Res<UnitAssets>,
    requests: Query<(Entity, &SpawnUnit)>,
) {
    for (request, spawn_unit) in requests.iter() {
        let unit_id = spawn_unit.0.clone();

        commands.spawn(Name::from(f!("{unit_id:?}")))
            .insert(RequireSprite(assets.crook.clone()))
        ;

        commands.entity(request)
            .despawn(); // TODO: Destroy Component, or kinda
    }
}