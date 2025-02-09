use crate::prelude::*;

pub trait TransformExt {
    fn with_splat_scale(self, value: f32) -> Self;
}

impl TransformExt for Transform {
    fn with_splat_scale(self, value: f32) -> Self {
        self.with_scale(Vec3 { x: value, y: value, z: 1.0 })
    }
}