use crate::prelude::*;
use crate::view::RequireSprite;

pub fn load_sprite(
    mut commands: Commands,
    entities: Query<(Entity, &RequireSprite)>,
) {
    for (entity, required_sprite) in entities.iter() {
        commands.entity(entity)
            .with_child(Sprite {
                image: required_sprite.0.clone(),
                ..default()
            })
            .remove::<RequireSprite>();
    }
}