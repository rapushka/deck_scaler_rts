use crate::gameplay::unit::view::UnitHeadView;
use crate::prelude::*;
use crate::prelude::selection::feature::SelectedUnit;

pub struct SelectionViewPlugin;

impl Plugin for SelectionViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, paint_selected_unit.in_set(Order::View))
        ;
    }
}

fn paint_selected_unit(
    units: Query<(&UnitHeadView, Option<&SelectedUnit>), With<UnitID>>,
    mut sprites: Query<&mut Sprite>,
) {
    for (head, selected) in units.iter() {
        let mut sprite = cq!(sprites.get_mut(head.0));

        let highlighted_color = Srgba::new(1.0, 1.0, 0.5, 1.0);

        sprite.color = match selected {
            None => Color::Srgba(Srgba::WHITE),
            Some(_) => Color::Srgba(highlighted_color),
        };
    }
}