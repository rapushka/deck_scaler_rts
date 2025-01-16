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