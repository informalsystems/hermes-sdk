use core::time::Duration;

use cgp::prelude::*;

#[cgp_component {
    name: TimeComponent,
    provider: ProvideTime,
    context: Runtime,
}]
pub trait HasTime: Async {
    type Time: Async;

    fn now(&self) -> Self::Time;

    fn duration_since(current_time: &Self::Time, other_time: &Self::Time) -> Duration;
}

pub type TimeOf<Runtime> = <Runtime as HasTime>::Time;
