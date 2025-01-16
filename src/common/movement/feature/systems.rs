use crate::common::movement::feature::components::Movement;
use crate::prelude::*;

pub fn handle_movement(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut Transform, &Movement)>,
    time: Res<Time>,
) {
    for (entity, mut transform, movement) in entities.iter_mut() {
        let position = &mut transform.translation;

        let movement = movement.0 * time.delta_secs();
        *position += movement.extend(0.0);

        commands.entity(entity)
            .remove::<Movement>();
    }
}