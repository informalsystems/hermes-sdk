use cgp_core::CanRaiseError;
use hermes_encoding_components::traits::decoder::CanDecode;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
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

use crate::impls::borsh_encode::ViaBorsh;
use crate::traits::json_rpc_client::HasJsonRpcClient;
use crate::types::commitment_proof::{JellyfishMerkleProof, SovereignCommitmentProof};
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
                slot_number: height.slot_number,
            })
                .into(),
        };

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

impl<Rollup, Counterparty, Encoding> RawClientStateWithProofsQuerier<Rollup, Counterparty>
    for QueryClientStateOnSovereign
where
    Rollup: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = RollupHeight>
        + HasRawClientStateType<RawClientState = Any>
        + HasCommitmentProofType<CommitmentProof = SovereignCommitmentProof>
        + HasJsonRpcClient
        + HasEncoding<Encoding = Encoding>
        + CanRaiseError<ClientError>
        + CanRaiseError<Encoding::Error>,
    Rollup::JsonRpcClient: ClientT,
    Encoding: HasEncodedType<Encoded = Vec<u8>> + CanDecode<ViaBorsh, JellyfishMerkleProof>,
{
    async fn query_raw_client_state_with_proofs(
        rollup: &Rollup,
        client_id: &ClientId,
        query_height: &RollupHeight,
    ) -> Result<(Any, SovereignCommitmentProof), Rollup::Error> {
        let request = Request {
            client_id: client_id.as_str(),
            query_height: &HeightParam {
                revision_number: 0,
                revision_height: query_height.slot_number,
            },
        };

        let response: QueryClientStateResponse = rollup
            .json_rpc_client()
            .request("ibc_clientState", (request,))
            .await
            .map_err(Rollup::raise_error)?;

        let client_state_any = Any {
            type_url: response.client_state.type_url,
            value: response.client_state.value,
        };

        let proof_bytes = response.proof;

        let merkle_proof = rollup
            .encoding()
            .decode(&proof_bytes)
            .map_err(Rollup::raise_error)?;

        let proof_height = RollupHeight {
            slot_number: response.proof_height.revision_height(),
        };

        let commitment_proof = SovereignCommitmentProof {
            proof_bytes,
            merkle_proof,
            proof_height,
        };

        Ok((client_state_any, commitment_proof))
    }
}

#[derive(Serialize)]
pub struct Request<'a> {
    pub client_id: &'a str,
    pub query_height: &'a HeightParam,
}
