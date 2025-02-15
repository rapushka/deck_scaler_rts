use crate::prelude::*;

pub fn spawn_test_level(
    mut commands: Commands,
) {
    let bg_size = Vec2::new(1_000.0, 500.0);

    commands.spawn(Name::from("bg"))
        .insert(Sprite {
            color: Color::Srgba(Srgba::rgb_u8(151, 151, 151)),
            custom_size: Some(bg_size),
            ..default()
        })
        .insert(Transform::from_translation(Vec3::Z * -10.0))
    ;
}