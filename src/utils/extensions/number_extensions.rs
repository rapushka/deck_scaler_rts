use crate::prelude::*;

pub trait NumberExt {
    fn to_seconds(self) -> Duration;

    fn to_timer(self) -> Timer;

    /// Clamps the lower bound. i.e. the self couldn't be lower than the arg
    fn clamp_min(self, min: Self) -> Self;

    /// Clamps the upper bound. i.e. the self couldn't be higher than the arg
    fn clamp_max(self, max: Self) -> Self;
}

impl NumberExt for f32 {
    fn to_seconds(self) -> Duration {
        Duration::from_secs_f32(self)
    }

    fn to_timer(self) -> Timer {
        Timer::from_seconds(self, TimerMode::Once)
    }

    fn clamp_min(self, min: Self) -> Self {
        self.max(min)
    }

    fn clamp_max(self, max: Self) -> Self {
        self.min(max)
    }
}