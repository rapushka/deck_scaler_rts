use crate::prelude::*;

/// This movement is NOT scaled by delta time
#[derive(Component)]
pub struct Movement(pub Vec2);