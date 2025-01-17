use crate::gameplay::unit::movement::feature::components::TargetPosition;
use crate::gameplay::unit::stats::MovementSpeed;
use crate::prelude::*;

pub fn move_unit_to_target(
    mut commands: Commands,
    units: Query<(Entity, &Transform, &TargetPosition, &MovementSpeed)>,
) {
    for (entity, transform, target, speed) in units.iter() {
        let position = transform.translation.xy();
        let target = target.0;
        let speed = speed.0;

        let direction = (target - position).normalize();

        commands.entity(entity)
            .insert(Movement(direction * speed))
        ;
    }
}

pub fn remove_target_position_if_too_close(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut Transform, &TargetPosition, &Movement)>,
    time: Res<Time>,
) {
    for (entity, mut transform, target, movement) in entities.iter_mut() {
        let position = &mut transform.translation;

        let target_position = target.0.extend(0.0);
        let movement = (movement.0 * time.delta_secs()).length();
        let distance = position.distance(target_position);

        if distance < movement {
            commands.entity(entity)
                .remove::<TargetPosition>()
            ;

            *position = target_position;
        }
    }
}