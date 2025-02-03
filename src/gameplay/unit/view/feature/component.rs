use crate::prelude::*;

#[derive(Component)]
pub struct LoadingView;

#[derive(Component)]
pub struct UnitHeadView(pub Entity);

#[derive(Component)]
pub struct UnitInfoContainer(pub Entity);