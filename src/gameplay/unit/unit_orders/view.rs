use crate::input::*;
use crate::prelude::*;

pub struct UnitOrdersViewPlugin;

impl Plugin for UnitOrdersViewPlugin {
    fn build(&self, app: &mut App) {
        app

            .add_systems(Update, view_unit_order.after(Order::UnitOrders))
        ;
    }
}

const ANIMATION_DURATION: f32 = 0.5;

fn view_unit_order(
    mut commands: Commands,
    orders: Query<&CursorWorldPosition, (With<PlayerInput>, With<JustClickedOrder>)>,
) {
    for cursor_position in orders.iter() {
        let cursor_position = cursor_position.0.extend(0.0);

        let transform = Transform::from_translation(cursor_position);
        let mut tmp = commands.spawn(Name::from("order pointer"));
        let view = tmp
            .insert(Sprite {
                color: Color::Srgba(Srgba::GREEN),
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..default()
            })
            .insert(transform)
            .insert(DestroyAfter(ANIMATION_DURATION.to_timer()))
            ;

        let target = view.id().into_target();
        let mut transform = target.transform_state(transform);

        view
            .animation()
            .length(ANIMATION_DURATION.to_seconds())
            .insert(
                tween::sequence((
                    DO(
                        ANIMATION_DURATION.to_seconds(),
                        EaseKind::BackIn,
                        transform.scale_to(Vec3::splat(0.0)),
                    ),
                ))
            )
        ;
    }
}
