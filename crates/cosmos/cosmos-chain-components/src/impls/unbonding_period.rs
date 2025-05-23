use core::time::Duration;

use hermes_core::relayer_components::chain::traits::{CanQueryChainHeight, HasHeightType};
use hermes_prelude::*;
use ibc::core::client::types::Height;
use ibc_proto::cosmos::staking::v1beta1::QueryParamsResponse;
use prost::{DecodeError, Message};

use crate::traits::{CanQueryAbci, UnbondingPeriodQuerier, UnbondingPeriodQuerierComponent};

pub struct StakingParamsUnbondingPeriod;

#[cgp_provider(UnbondingPeriodQuerierComponent)]
impl<Chain> UnbondingPeriodQuerier<Chain> for StakingParamsUnbondingPeriod
where
    Chain: CanQueryChainHeight
        + CanQueryAbci
        + HasHeightType<Height = Height>
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<DecodeError>,
{
    async fn query_unbonding_period(chain: &Chain) -> Result<Duration, Chain::Error> {
        let latest_height = chain.query_chain_height().await?;

        let query_staking_params_bytes = chain
            .query_abci(
                "/cosmos.staking.v1beta1.Query/Params",
                &"".to_owned().into_bytes(),
                &latest_height,
            )
            .await?
            .ok_or_else(|| Chain::raise_error("failed to query for staking params"))?;

        let query_staking_params: QueryParamsResponse =
            QueryParamsResponse::decode(query_staking_params_bytes.as_ref())
                .map_err(Chain::raise_error)?;

        let staking_params = query_staking_params
            .params
            .ok_or_else(|| Chain::raise_error("staking params is empty"))?;

        let unbonding_time = staking_params
            .unbonding_time
            .ok_or_else(|| Chain::raise_error("unbonding time in staking params is empty"))?;

        Ok(Duration::new(
            unbonding_time.seconds as u64,
            unbonding_time.nanos as u32,
        ))
    }
}
