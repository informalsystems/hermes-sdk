use core::time::Duration;
use std::time::Instant;

use cgp::prelude::Async;
use hermes_runtime_components::traits::time::ProvideTime;

pub struct ProvideStdTime;

impl<Runtime> ProvideTime<Runtime> for ProvideStdTime
where
    Runtime: Async,
{
    type Time = Instant;

    fn now(_runtime: &Runtime) -> Instant {
        Instant::now()
    }

    fn duration_since(time: &Instant, other: &Instant) -> Duration {
        time.duration_since(*other)
    }
}
