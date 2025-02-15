use crate::common::destroy_after::DestroyAfter;
use crate::prelude::*;

pub fn destroy_after_delay(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut DestroyAfter)>,
    time: Res<Time<Virtual>>,
) {
    for (entity, mut destroy_after) in entities.iter_mut() {
        destroy_after.0.tick(time.delta());

        if !destroy_after.0.finished() {
            continue;
        }

        commands.entity(entity).despawn_recursive();
    }
}