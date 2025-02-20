use core::time::Duration;

use cgp::prelude::*;

#[cgp_component {
    provider: PollIntervalGetter,
    context: Chain,
  }]
pub trait HasPollInterval {
    fn poll_interval(&self) -> Duration;
}
