use hermes_prelude::*;

use crate::traits::HasTimeType;

#[cgp_component {
  name: TimeoutTypeComponent,
  provider: ProvideTimeoutType,
  context: Chain,
}]
pub trait HasTimeoutType: HasTimeType {
    type Timeout: Async;

    fn has_timed_out(time: &Self::Time, timeout: &Self::Timeout) -> bool;
}

pub type TimeoutOf<Chain> = <Chain as HasTimeoutType>::Timeout;
