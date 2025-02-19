use crate::input::{CursorWorldPosition, JustClickedTarget, JustClickedSelect, PlayerInput};
use crate::prelude::*;

pub fn handle_select_clicks(
    entities: Query<(Entity, &Transform, &CircleCollider)>,
    clicks: Query<&CursorWorldPosition, (With<PlayerInput>, With<JustClickedSelect>)>,
    mut event: EventWriter<ClickSelect>,
) {
    for cursor_position in clicks.iter() {
        for (entity, transform, collider) in entities.iter() {
            let cursor_position = cursor_position.0;
            let entity_position = transform.translation.truncate();

            let distance = cursor_position.distance(entity_position);

            if distance > collider.radius() {
                continue;
            }

            event.send(ClickSelect(entity));
        }
    }
}

pub fn handle_target_clicks(
    entities: Query<(Entity, &Transform, &CircleCollider)>,
    clicks: Query<&CursorWorldPosition, (With<PlayerInput>, With<JustClickedTarget>)>,
    mut click_on_unit_event: EventWriter<ClickTargetUnit>,
    mut click_on_position_event: EventWriter<ClickTargetPosition>,
) {
    for cursor_position in clicks.iter() {
        let cursor_position = cursor_position.0;
        let target = find_entity_on_position(&entities, cursor_position);

        match target {
            Some(e) => { click_on_unit_event.send(ClickTargetUnit(e)); }
            None => { click_on_position_event.send(ClickTargetPosition(cursor_position)); }
        };
    }
}

fn find_entity_on_position(
    entities: &Query<(Entity, &Transform, &CircleCollider)>,
    target_position: Vec2,
) -> Option<Entity> {
    for (entity, transform, collider) in entities.iter() {
        let entity_position = transform.translation.truncate();
        let distance = target_position.distance(entity_position);

        if distance <= collider.radius() {
            return Some(entity);
        }
    }

    None
}