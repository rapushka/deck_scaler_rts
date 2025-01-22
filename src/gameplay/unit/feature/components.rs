use crate::gameplay::unit::side::feature::Side;
use crate::prelude::*;

#[derive(Component, Debug, Copy, Clone)]
pub enum UnitID {
    Crook,
    Rat,
}

#[derive(Event)]
pub struct SpawnUnit {
    pub id: UnitID,
    pub position: Vec2,
    pub side: Side,
}