use crate::prelude::*;

#[derive(Component, Debug, Clone)]
pub enum UnitID {
    Crook,
}

#[derive(Component)]
pub struct SpawnUnit(pub UnitID);