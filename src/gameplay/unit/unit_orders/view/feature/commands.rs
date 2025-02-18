use crate::assets::UiAssets;
use crate::prelude::*;

const ANIMATION_DURATION: f32 = 0.3;

pub struct SpawnOrderViewCommand {
    pub position: Vec2,
    pub scale: f32,
    pub color: Srgba,
}

impl Command for SpawnOrderViewCommand {
    fn apply(self, world: &mut World) {
        let arrow_image = world.resource::<UiAssets>().arrows.clone();
        let position = self.position.extend(10.0);

        let transform = Transform {
            translation: position,
            scale: Vec3::splat(self.scale),
            rotation: Quat::from_rotation_z(45.0_f32.to_radians()),
        };

        let mut commands = world.commands();
        let mut temporary_value = commands.spawn(Name::from("order pointer"));
        let view = temporary_value
            .insert(Sprite {
                image: arrow_image,
                color: Color::Srgba(self.color.with_alpha(0.5)),
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