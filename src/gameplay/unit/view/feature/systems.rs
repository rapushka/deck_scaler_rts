use crate::assets::UnitAssets;
use crate::gameplay::unit::view::{LoadingView, UnitInfoContainer, UnitHeadView};
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
            .insert(Visibility::default())
            .id();

        let head = commands.spawn(Name::from("head"))
            .insert(Transform::default())
            .insert(Visibility::default())
            .insert(Sprite::from_image(assets.get_head(unit_id)))
            .id();

        commands.entity(unit)
            .add_child(info_container)
            .insert(UnitInfoContainer(info_container))

            .add_child(head)
            .insert(UnitHeadView(head))

            .remove::<LoadingView>()
        ;
    }
}