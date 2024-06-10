use cgp_core::CanRaiseError;
use hermes_protobuf_encoding_components::types::Any;
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    RawConsensusStateQuerier, RawConsensusStateWithProofsQuerier,
};
use hermes_relayer_components::chain::traits::types::consensus_state::HasRawConsensusStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightFields;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc::core::client::types::error::ClientError as Ics02Error;
use ibc::core::host::types::error::IdentifierError;
use ibc_query::core::client::QueryConsensusStateResponse;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::ClientError;
use serde::Serialize;

use crate::traits::json_rpc_client::HasJsonRpcClient;
use crate::types::height::RollupHeight;
use crate::types::rpc::height::HeightParam;

pub struct QueryConsensusStateOnSovereign;

impl<Rollup, Counterparty> RawConsensusStateQuerier<Rollup, Counterparty>
    for QueryConsensusStateOnSovereign
where
    Rollup: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = RollupHeight>
        + HasRawConsensusStateType<RawConsensusState = Any>
        + HasJsonRpcClient
        + CanRaiseError<ClientError>
        + CanRaiseError<Ics02Error>
        + CanRaiseError<IdentifierError>,
    Rollup::JsonRpcClient: ClientT,
    Counterparty: HasHeightFields,
{
    async fn query_raw_consensus_state(
        rollup: &Rollup,
        client_id: &ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &RollupHeight,
    ) -> Result<Any, Rollup::Error> {
        let request = Request {
            client_id: client_id.as_str(),
            consensus_height: &HeightParam {
                revision_number: Counterparty::revision_number(consensus_height),
                revision_height: Counterparty::revision_height(consensus_height),
            },
            query_height: &(&RollupHeight {
                slot_number: query_height.slot_number + 2,
            })
                .into(),
        };

        let response: QueryConsensusStateResponse = rollup
            .json_rpc_client()
            .request("ibc_consensusState", (request,))
            .await
            .map_err(Rollup::raise_error)?;

        Ok(Any {
            type_url: response.consensus_state.type_url,
            value: response.consensus_state.value,
        })
    }
}

impl<Rollup, Counterparty> RawConsensusStateWithProofsQuerier<Rollup, Counterparty>
    for QueryConsensusStateOnSovereign
where
    Rollup: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = RollupHeight>
        + HasRawConsensusStateType<RawConsensusState = Any>
        + HasCommitmentProofType<CommitmentProof = Vec<u8>>
        + HasJsonRpcClient
        + CanRaiseError<ClientError>
        + CanRaiseError<Ics02Error>
        + CanRaiseError<IdentifierError>,
    Rollup::JsonRpcClient: ClientT,
    Counterparty: HasHeightFields,
{
    async fn query_raw_consensus_state_with_proofs(
        rollup: &Rollup,
        client_id: &ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &RollupHeight,
    ) -> Result<(Any, Vec<u8>), Rollup::Error> {
        let request = Request {
            client_id: client_id.as_str(),
            consensus_height: &HeightParam {
                revision_number: Counterparty::revision_number(consensus_height),
                revision_height: Counterparty::revision_height(consensus_height),
            },
            query_height: &(&RollupHeight {
                slot_number: query_height.slot_number + 2,
            })
                .into(),
        };

        let response: QueryConsensusStateResponse = rollup
            .json_rpc_client()
            .request("ibc_consensusState", (request,))
            .await
            .map_err(Rollup::raise_error)?;

        Ok((
            Any {
                type_url: response.consensus_state.type_url,
                value: response.consensus_state.value,
            },
            response.proof,
        ))
    }
}

#[derive(Serialize)]
pub struct Request<'a> {
    pub client_id: &'a str,
    pub consensus_height: &'a HeightParam,
    pub query_height: &'a HeightParam,
}
