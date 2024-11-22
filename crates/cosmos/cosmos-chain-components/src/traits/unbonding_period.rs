use cgp::prelude::*;

#[derive_component(UnbondingPeriodQuerierComponent, UnbondingPeriodQuerier<Chain>)]
#[async_trait]
pub trait CanQueryUnbondingPeriod: HasErrorType + Async {
    type UnbondingPeriod;

    async fn query_unbonding_period(&self) -> Result<Self::UnbondingPeriod, Self::Error>;
}
