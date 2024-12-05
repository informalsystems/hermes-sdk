use core::time::Duration;

use cgp::prelude::*;

#[derive_component(TimeComponent, ProvideTime<Runtime>)]
pub trait HasTime: Async {
    type Time: Async;

    fn now(&self) -> Self::Time;

    fn duration_since(current_time: &Self::Time, other_time: &Self::Time) -> Duration;
}

pub type TimeOf<Runtime> = <Runtime as HasTime>::Time;
