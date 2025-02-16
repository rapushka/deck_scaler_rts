use crate::gameplay::unit::side::feature::OnSide;
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
    pub side: OnSide,
    pub is_lead: bool,
}