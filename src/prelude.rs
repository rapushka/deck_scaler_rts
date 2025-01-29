#![allow(dead_code)]
#![allow(unused_imports)]

// externals
pub use bevy::prelude::*;
pub use tiny_bail::prelude::*;
pub use bevy_tween::prelude::*;

// internals
pub use crate::infrastructure::app_state::*;
pub use crate::infrastructure::order::*;
pub use crate::gameplay::unit::*;
pub use crate::gameplay::affect::*;
pub use crate::common::*;
pub use crate::utils::*;

// aliases
pub use std::format as f;
pub use bevy_tween::tween::AnimationTarget as TweenTarget;
pub use bevy_tween::combinator as tween;
pub use bevy_tween::combinator::tween as DO;