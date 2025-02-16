use crate::gameplay::unit::side::feature::{OnEnemySide, OnPlayerSide, OnSide};
use crate::prelude::*;

pub fn on_side_added(
    mut commands: Commands,
    units: Query<(Entity, &OnSide), Added<OnSide>>,
) {
    for (unit, side) in units.iter() {
        let mut entity = commands.entity(unit);

        match side {
            OnSide::Player => entity.insert(OnPlayerSide),
            OnSide::Enemy => entity.insert(OnEnemySide),
        };
    }
}