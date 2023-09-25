use core::time::Duration;

use cgp_core::traits::Async;

pub trait HasTime: Async {
    type Time: Async;

    fn now(&self) -> Self::Time;

    fn duration_since(current_time: &Self::Time, other_time: &Self::Time) -> Duration;
}
