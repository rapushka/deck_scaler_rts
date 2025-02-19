use bevy::ecs::component::StorageType;
use crate::prelude::*;

#[derive(Component)]
pub struct PlayerInput;

#[derive(Component)]
pub struct CursorWorldPosition(pub Vec2);

pub struct JustClickedSelect;

impl Component for JustClickedSelect {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
}
pub struct JustClickedTarget;

impl Component for JustClickedTarget {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CursorScreenPosition {
    pub current: Vec2,
    pub previous: Vec2,
}

impl CursorScreenPosition {
    pub fn new() -> Self {
        CursorScreenPosition {
            current: Vec2::ZERO,
            previous: Vec2::ZERO,
        }
    }
}

impl CursorScreenPosition {
    pub fn delta(&self) -> Vec2 {
        Vec2 {
            x: self.previous.x - self.current.x,
            y: self.current.y - self.previous.y,
        }
    }
}
