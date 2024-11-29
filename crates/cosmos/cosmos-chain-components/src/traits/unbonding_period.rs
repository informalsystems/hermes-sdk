use core::time::Duration;

use cgp::prelude::*;

#[derive_component(UnbondingPeriodQuerierComponent, UnbondingPeriodQuerier<Chain>)]
#[async_trait]
pub trait CanQueryUnbondingPeriod: HasErrorType + Async {
    async fn query_unbonding_period(&self) -> Result<Duration, Self::Error>;
}
