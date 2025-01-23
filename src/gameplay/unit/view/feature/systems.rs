use crate::assets::UnitAssets;
use crate::gameplay::unit::view::{LoadingView, UnitInfoContainer};
use crate::prelude::*;
use bevy::prelude::{Entity, Query, With};

pub fn load_unit_view(
    mut commands: Commands,
    units: Query<(Entity, &UnitID), With<LoadingView>>,
    assets: Res<UnitAssets>,
) {
    for (unit, unit_id) in units.iter() {
        let info_container = commands.spawn(Name::from("info"))
            .insert(Transform::from_translation(Vec3 { x: 0.0, y: -55.0, z: 10.0 }))
            .set_parent(unit)
            .id();

        let sprite = match unit_id {
            UnitID::Crook => assets.crook.clone(),
            UnitID::Rat => assets.rat.clone(),
        };

        commands.entity(unit)
            .insert(Sprite::from_image(sprite))
            .insert(UnitInfoContainer(info_container))

            .remove::<LoadingView>()
        ;
    }
}