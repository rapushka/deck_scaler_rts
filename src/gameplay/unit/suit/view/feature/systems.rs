use crate::assets::SuitAssets;
use crate::gameplay::unit::suit::view::SuitView;
use crate::gameplay::unit::view::UnitInfoContainer;
use crate::prelude::*;

pub fn spawn_suit_view(
    mut commands: Commands,
    units: Query<(Entity, &Suit, &UnitInfoContainer), Without<SuitView>>,
    assets: Res<SuitAssets>,
) {
    for (entity, suit, container) in units.iter() {
        let image = assets.get_sprite(suit);

        let view = commands.spawn(Name::new("Suit View"))
            .set_parent(container.0)
            .insert(Sprite {
                image,
                ..default()
            })
            .id();

        commands.entity(entity)
            .insert(SuitView(view));
    }
}

pub fn update_suit_view(
    units: Query<(&SuitView, &Suit), Changed<Suit>>,
    mut sprites: Query<&mut Sprite>,
    assets: Res<SuitAssets>,
) {
    for (SuitView(view), suit) in units.iter() {
        let mut sprite = sprites.get_mut(*view).expect("Suit View must have Sprite");
        sprite.image = assets.get_sprite(suit);
    }
}