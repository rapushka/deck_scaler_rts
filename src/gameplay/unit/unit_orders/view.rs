use crate::assets::UiAssets;
use crate::input::*;
use crate::prelude::*;

pub struct UnitOrdersViewPlugin;

impl Plugin for UnitOrdersViewPlugin {
    fn build(&self, app: &mut App) {
        app

            .add_systems(Update, view_unit_order
                .run_if(in_state(AppState::Gameplay))
                .after(Order::UnitOrders))
        ;
    }
}

const ANIMATION_DURATION: f32 = 0.3;

fn view_unit_order(
    mut commands: Commands,
    orders: Query<&CursorWorldPosition, (With<PlayerInput>, With<JustClickedTarget>)>,
    assets: Res<UiAssets>,
) {
    for cursor_position in orders.iter() {
        let cursor_position = cursor_position.0.extend(10.0);

        let transform = Transform {
            translation: cursor_position,
            scale: Vec3::splat(0.4),
            rotation: Quat::from_rotation_z(45.0_f32.to_radians()),
        };

        let mut tmp = commands.spawn(Name::from("order pointer")); //
        let view = tmp
            .insert(Sprite {
                image: assets.arrows.clone(),
                color: Color::Srgba(Srgba::GREEN.with_alpha(0.5)),
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
                tween::parallel((
                    DO(
                        ANIMATION_DURATION.to_seconds(),
                        EaseKind::BackIn,
                        transform.scale_to(Vec3::splat(0.0)),
                    ),
                    // DO(
                    //     ANIMATION_DURATION.to_seconds(),
                    //     EaseKind::BackIn,
                    //     transform.rotation_to(Quat::from_rotation_z(-1.0)),
                    // ),
                ))
            )
        ;
    }
}
