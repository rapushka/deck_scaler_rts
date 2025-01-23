use crate::prelude::*;
use crate::prelude::selection::feature::SelectedUnit;

pub struct SelectionViewPlugin;

impl Plugin for SelectionViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, paint_selected_unit.in_set(Order::Postprocess))
        ;
    }
}

fn paint_selected_unit(
    mut units: Query<(&mut Sprite, Option<&SelectedUnit>), With<UnitID>>,
) {
    for (mut sprite, selected) in units.iter_mut() {
        let highlighted_color = Srgba::new(1.0, 1.0, 0.5, 1.0);

        sprite.color = match selected {
            None => Color::Srgba(Srgba::WHITE),
            Some(_) => Color::Srgba(highlighted_color),
        };
    }
}