use crate::input::{CursorWorldPosition, JustClickedSelect, PlayerInput};
use crate::prelude::*;

pub fn handle_clicks(
    entities: Query<(Entity, &Transform, &CircleCollider)>,
    clicks: Query<&CursorWorldPosition, (With<PlayerInput>, With<JustClickedSelect>)>,
    mut event: EventWriter<Clicked>,
) {
    for (entity, transform, collider) in entities.iter() {
        for cursor_position in clicks.iter() {
            let cursor_position = cursor_position.0;
            let entity_position = transform.translation.truncate();

            let distance = cursor_position.distance(entity_position);

            if distance > collider.radius() {
                continue;
            }

            event.send(Clicked(entity));
        }
    }
}