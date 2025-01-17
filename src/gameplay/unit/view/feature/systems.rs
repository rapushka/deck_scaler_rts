use crate::assets::UnitAssets;
use crate::gameplay::unit::view::LoadingView;
use crate::prelude::*;
use bevy::prelude::{Entity, Query, With};

pub fn load_unit_view(
    mut commands: Commands,
    units: Query<(Entity, &UnitID), With<LoadingView>>,
    assets: Res<UnitAssets>,
) {
    for (unit, unit_id) in units.iter() {
        let sprite = match unit_id {
            UnitID::Crook => assets.crook.clone()
        };

        commands.entity(unit)
            .insert(Sprite::from_image(sprite))
            .remove::<LoadingView>()
        ;
    }
}