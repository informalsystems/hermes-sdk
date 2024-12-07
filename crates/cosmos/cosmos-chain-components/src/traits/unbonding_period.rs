use core::time::Duration;

use cgp::prelude::*;

#[cgp_component {
  name: UnbondingPeriodQuerierComponent,
  provider: UnbondingPeriodQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryUnbondingPeriod: HasErrorType + Async {
    async fn query_unbonding_period(&self) -> Result<Duration, Self::Error>;
}
