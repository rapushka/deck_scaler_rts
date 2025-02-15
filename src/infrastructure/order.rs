use crate::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Order {
    Input,

    UnitOrders,
    SelectUnits,

    UpdateStats,
    GameLogic,

    ChargeAttack,
    CreateAffects,

    MoveUnits,
    InfluenceAffects,

    ApplyAffects,

    View,

    Cleanups,
}