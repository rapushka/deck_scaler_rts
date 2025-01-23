use crate::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Order {
    Input,

    Preprocess,
    GameLogic,
    Postprocess,

    View,

    Cleanups,
}