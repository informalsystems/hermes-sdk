use cgp::prelude::*;

use crate::traits::time::HasTimeType;

#[derive_component(TimeoutTypeComponent, ProvideTimeoutType<Chain>)]
pub trait HasTimeoutType: HasTimeType {
    type Timeout: Async;

    fn has_timed_out(time: &Self::Time, timeout: &Self::Timeout) -> bool;
}
