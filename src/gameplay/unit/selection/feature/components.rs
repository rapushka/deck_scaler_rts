use crate::prelude::*;

#[derive(Component)]
pub struct SelectedUnit;

#[derive(Event)]
pub struct MarkUnitSelected(pub Entity);