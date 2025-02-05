use crate::gameplay::affect::Affect;
use crate::prelude::*;

pub fn destroy_affects(
    mut commands: Commands,
    affects: Query<Entity, With<Affect>>,
) {
    for affect in affects.iter() {
        commands.entity(affect).despawn_recursive();
    }
}