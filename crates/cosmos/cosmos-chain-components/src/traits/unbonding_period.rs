use core::time::Duration;

use hermes_prelude::*;

#[cgp_component {
  provider: UnbondingPeriodQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryUnbondingPeriod: HasAsyncErrorType + Async {
    async fn query_unbonding_period(&self) -> Result<Duration, Self::Error>;
}
