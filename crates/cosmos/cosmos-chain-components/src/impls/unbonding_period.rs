use cgp::prelude::*;
use core::time::Duration;
use eyre::Report;
use prost::{DecodeError, Message};
use prost_types::Any;

use hermes_chain_type_components::traits::types::commitment_proof::HasCommitmentProofType;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_encoding_components::types::AsBytes;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;

use ibc_proto::cosmos::staking::v1beta1::QueryParamsResponse;
use ibc_relayer_types::Height;

use crate::traits::abci_query::CanQueryAbci;
use crate::traits::unbonding_period::UnbondingPeriodQuerier;
use crate::types::commitment_proof::CosmosCommitmentProof;

pub struct StakingParamsUnbondingPeriod;

#[async_trait]
impl<Chain, Encoding> UnbondingPeriodQuerier<Chain> for StakingParamsUnbondingPeriod
where
    Chain: CanQueryChainHeight
        + CanQueryAbci
        + HasHeightType<Height = Height>
        + HasCommitmentProofType<CommitmentProof = CosmosCommitmentProof>
        + HasDefaultEncoding<AsBytes, Encoding = Encoding>
        + CanRaiseError<Report>
        + CanRaiseError<Encoding::Error>
        + CanRaiseError<DecodeError>,
    Encoding: Async + CanConvert<Any, QueryParamsResponse>,
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

        let query_params_any: Any =
            Message::decode(query_staking_params_bytes.as_ref()).map_err(Chain::raise_error)?;

        let query_staking_params = Chain::default_encoding()
            .convert(&query_params_any)
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
