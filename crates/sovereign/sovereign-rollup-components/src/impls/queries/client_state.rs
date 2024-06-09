use cgp_core::CanRaiseError;
use hermes_protobuf_encoding_components::types::Any;
use hermes_relayer_components::chain::traits::queries::client_state::{
    RawClientStateQuerier, RawClientStateWithProofsQuerier,
};
use hermes_relayer_components::chain::traits::types::client_state::HasRawClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc_query::core::client::QueryClientStateResponse;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::ClientError;
use serde::Serialize;

use crate::traits::json_rpc_client::HasJsonRpcClient;
use crate::types::height::RollupHeight;
use crate::types::rpc::height::HeightParam;

pub struct QueryClientStateOnSovereign;

impl<Rollup, Counterparty> RawClientStateQuerier<Rollup, Counterparty>
    for QueryClientStateOnSovereign
where
    Rollup: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = RollupHeight>
        + HasRawClientStateType<RawClientState = Any>
        + HasJsonRpcClient
        + CanRaiseError<ClientError>,
    Rollup::JsonRpcClient: ClientT,
{
    async fn query_raw_client_state(
        rollup: &Rollup,
        client_id: &ClientId,
        height: &RollupHeight,
    ) -> Result<Any, Rollup::Error> {
        let request = Request {
            client_id: client_id.as_str(),
            query_height: &(&RollupHeight {
                slot_number: height.slot_number + 2,
            })
                .into(),
        };

        std::thread::sleep(std::time::Duration::from_secs(2));

        let response: QueryClientStateResponse = rollup
            .json_rpc_client()
            .request("ibc_clientState", (request,))
            .await
            .map_err(Rollup::raise_error)?;

        Ok(Any {
            type_url: response.client_state.type_url,
            value: response.client_state.value,
        })
    }
}

impl<Rollup, Counterparty> RawClientStateWithProofsQuerier<Rollup, Counterparty>
    for QueryClientStateOnSovereign
where
    Rollup: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = RollupHeight>
        + HasRawClientStateType<RawClientState = Any>
        + HasCommitmentProofType<CommitmentProof = Vec<u8>>
        + HasJsonRpcClient
        + CanRaiseError<ClientError>,
    Rollup::JsonRpcClient: ClientT,
{
    async fn query_raw_client_state_with_proofs(
        rollup: &Rollup,
        client_id: &ClientId,
        height: &RollupHeight,
    ) -> Result<(Any, Vec<u8>), Rollup::Error> {
        let request = Request {
            client_id: client_id.as_str(),
            query_height: &(&RollupHeight {
                slot_number: height.slot_number + 2,
            })
                .into(),
        };

        std::thread::sleep(std::time::Duration::from_secs(2));

        let response: QueryClientStateResponse = rollup
            .json_rpc_client()
            .request("ibc_clientState", (request,))
            .await
            .map_err(Rollup::raise_error)?;

        Ok((
            Any {
                type_url: response.client_state.type_url,
                value: response.client_state.value,
            },
            response.proof,
        ))
    }
}

#[derive(Serialize)]
pub struct Request<'a> {
    pub client_id: &'a str,
    pub query_height: &'a HeightParam,
}
