use crate::prelude::*;

pub trait NumberExt {
    fn to_seconds(self) -> Duration;

    fn to_timer(self) -> Timer;
}

impl NumberExt for f32 {
    fn to_seconds(self) -> Duration {
        Duration::from_secs_f32(self)
    }

    fn to_timer(self) -> Timer {
        Timer::from_seconds(self, TimerMode::Once)
    }
}