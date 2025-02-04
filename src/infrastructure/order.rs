use crate::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Order {
    Input,

    UnitOrders,

    Preprocess,
    GameLogic,
    Postprocess,

    ApplyAffects,

    View,

    Cleanups,
}