use crate::gameplay::unit::side::feature::{OnEnemySide, OnPlayerSide, Side};
use crate::prelude::*;

pub fn on_side_added(
    mut commands: Commands,
    units: Query<(Entity, &Side), Added<Side>>,
) {
    for (unit, side) in units.iter() {
        let mut entity = commands.entity(unit);

        match side {
            Side::Player => entity.insert(OnPlayerSide),
            Side::Enemy => entity.insert(OnEnemySide),
        };
    }
}