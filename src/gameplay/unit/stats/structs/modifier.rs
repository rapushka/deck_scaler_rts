use crate::prelude::*;

#[derive(Reflect, Copy, Clone)]
pub struct Modifier {
    multiplier: f32,
    additive: f32,
}

impl Default for Modifier {
    fn default() -> Self {
        Modifier {
            multiplier: 1.0,
            additive: 0.0,
        }
    }
}

impl Modifier {
    pub fn multiply(value: f32) -> Self {
        Modifier {
            multiplier: value,
            ..default()
        }
    }

    pub fn add(value: f32) -> Self {
        Modifier {
            additive: value,
            ..default()
        }
    }

    pub fn modify(&self, source: f32) -> f32 {
        (source + self.additive) * self.multiplier
    }
}