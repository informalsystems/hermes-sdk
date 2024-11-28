use cgp::prelude::*;
use core::time::Duration;
use eyre::Report;
use prost::{DecodeError, Message};

use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;

use ibc_proto::cosmos::staking::v1beta1::QueryParamsResponse;
use ibc_relayer_types::Height;

use crate::traits::abci_query::CanQueryAbci;
use crate::traits::unbonding_period::UnbondingPeriodQuerier;

pub struct StakingParamsUnbondingPeriod;

#[async_trait]
impl<Chain> UnbondingPeriodQuerier<Chain> for StakingParamsUnbondingPeriod
where
    Chain: CanQueryChainHeight
        + CanQueryAbci
        + HasHeightType<Height = Height>
        + CanRaiseError<Report>
        + CanRaiseError<DecodeError>,
{
    type UnbondingPeriod = Duration;

    async fn query_unbonding_period(chain: &Chain) -> Result<Self::UnbondingPeriod, Chain::Error> {
        let latest_height = chain.query_chain_height().await?;

        let query_staking_params_bytes = chain
            .query_abci(
                "/cosmos.staking.v1beta1.Query/Params",
                &"".to_owned().into_bytes(),
                &latest_height,
            )
            .await?;

        let query_staking_params: QueryParamsResponse =
            QueryParamsResponse::decode(query_staking_params_bytes.as_ref())
                .map_err(Chain::raise_error)?;

        let staking_params = query_staking_params
            .params
            .ok_or_else(|| Report::msg("staking params is empty"))
            .map_err(Chain::raise_error)?;

        let unbonding_time = staking_params
            .unbonding_time
            .ok_or_else(|| Report::msg("unbonding time in staking params is empty"))
            .map_err(Chain::raise_error)?;

        Ok(Duration::new(
            unbonding_time.seconds as u64,
            unbonding_time.nanos as u32,
        ))
    }
}
