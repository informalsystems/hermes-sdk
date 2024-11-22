use cgp::core::error::CanRaiseError;
use cgp::core::Async;
use eyre::Report;
use prost::{DecodeError, Message};
use prost_types::Any;

use ibc_proto::cosmos::staking::v1beta1::{Params, QueryParamsResponse};
use ibc_relayer_types::Height;

use hermes_chain_type_components::traits::types::commitment_proof::HasCommitmentProofType;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_encoding_components::types::AsBytes;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;

use crate::traits::abci_query::CanQueryAbci;
use crate::traits::staking_params_query::StakingParamsQuerier;
use crate::types::commitment_proof::CosmosCommitmentProof;

pub struct QueryStakingParams;

impl<Chain, Encoding> StakingParamsQuerier<Chain> for QueryStakingParams
where
    Chain: CanQueryAbci
        + HasHeightType<Height = Height>
        + HasCommitmentProofType<CommitmentProof = CosmosCommitmentProof>
        + HasDefaultEncoding<AsBytes, Encoding = Encoding>
        + CanRaiseError<Report>
        + CanRaiseError<Encoding::Error>
        + CanRaiseError<DecodeError>,
    Encoding: Async + CanConvert<Any, QueryParamsResponse>,
{
    async fn query_staking_params(chain: &Chain, height: &Height) -> Result<Params, Chain::Error> {
        let query_staking_params_bytes = chain
            .query_abci(
                "/cosmos.staking.v1beta1.Query/Params",
                &"".to_owned().into_bytes(),
                height,
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

        Ok(staking_params)
    }

    async fn query_staking_params_with_proofs(
        chain: &Chain,
        height: &Height,
    ) -> Result<(Params, Chain::CommitmentProof), Chain::Error> {
        let (query_staking_params_bytes, commitment_proof) = chain
            .query_abci_with_proofs(
                "/cosmos.staking.v1beta1.Query/Params",
                &"".to_owned().into_bytes(),
                height,
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

        Ok((staking_params, commitment_proof))
    }
}
