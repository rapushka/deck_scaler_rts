use crate::prelude::*;

#[derive(Component)]
pub struct PlayerInput;

#[derive(Component)]
pub struct CursorPosition(pub Vec2);

#[derive(Component)]
pub struct JustClickedSelect;

#[derive(Component)]
pub struct JustClickedOrder;

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
