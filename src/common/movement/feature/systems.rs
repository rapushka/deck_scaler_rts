use crate::prelude::*;

pub fn move_to_target(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut Transform, &TargetPosition, &MovementSpeed)>,
    time: Res<Time>,
) {
    for (entity, mut transform, target, speed) in entities.iter_mut() {
        let position = &mut transform.translation;
        let target = target.0.extend(0.0);
        let speed = speed.0;

        let direction = (target - *position).normalize();
        let distance = target.distance(*position);
        let delta = speed * time.delta_secs();

        if delta >= distance {
            *position = target;
            commands.entity(entity)
                .remove::<TargetPosition>()
            ;

            continue;
        }

        *position += direction * delta;
    }
}