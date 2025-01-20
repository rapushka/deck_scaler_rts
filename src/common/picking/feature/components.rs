use crate::prelude::*;

#[derive(Event)]
pub struct Clicked(pub Entity);

#[derive(Component)]
pub struct CircleCollider {
    radius: f32,
}

impl CircleCollider {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}