use core::time::Duration;

use hermes_prelude::*;

#[cgp_component {
  provider: ClientRefreshRateGetter,
  context: Chain,
}]
pub trait HasClientRefreshRate {
    fn client_refresh_rate(&self) -> &Option<Duration>;
}
