/*!
   Trait definition for [`HasTimestampType`].
*/

use core::time::Duration;

use cgp::prelude::*;

#[derive_component(TimeTypeComponent, ProvideTimeType<Chain>)]
pub trait HasTimeType: Async {
    type Time: Async;

    fn duration_since(earlier: &Self::Time, later: &Self::Time) -> Option<Duration>;
}

#[derive_component(TimeoutTypeComponent, ProvideTimeoutType<Chain>)]
pub trait HasTimeoutType: HasTimeType {
    type Timeout: Async;

    fn has_timed_out(time: &Self::Time, timeout: &Self::Timeout) -> bool;
}
