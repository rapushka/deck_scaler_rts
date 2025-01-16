use crate::prelude::*;

#[derive(Component, Debug, Clone)]
pub enum UnitID {
    Crook,
}

#[derive(Event)]
pub struct SpawnUnit(pub UnitID);