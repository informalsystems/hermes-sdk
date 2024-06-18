use cgp_core::CanRaiseError;
use hermes_encoding_components::traits::decoder::CanDecode;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_relayer_components::chain::traits::queries::connection_end::{
    ConnectionEndQuerier, ConnectionEndWithProofsQuerier,
};
use hermes_relayer_components::chain::traits::types::connection::HasConnectionEndType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc::core::connection::types::ConnectionEnd;
use ibc_query::core::connection::QueryConnectionResponse;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::ClientError;
use serde::Serialize;

use crate::impls::borsh_encode::ViaBorsh;
use crate::traits::json_rpc_client::HasJsonRpcClient;
use crate::types::commitment_proof::{JellyfishMerkleProof, SovereignCommitmentProof};
use crate::types::height::RollupHeight;
use crate::types::rpc::height::HeightParam;

pub struct QueryConnectionEndOnSovereign;

impl<Rollup, Counterparty> ConnectionEndQuerier<Rollup, Counterparty>
    for QueryConnectionEndOnSovereign
where
    Rollup: HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>
        + HasIbcChainTypes<Counterparty, Height = RollupHeight>
        + HasJsonRpcClient
        + CanRaiseError<ClientError>,
    Rollup::JsonRpcClient: ClientT,
{
    async fn query_connection_end(
        rollup: &Rollup,
        connection_id: &Rollup::ConnectionId,
        height: &Rollup::Height,
    ) -> Result<Rollup::ConnectionEnd, Rollup::Error> {
        let request = Request {
            connection_id: &connection_id.to_string(),
            query_height: &(&RollupHeight {
                slot_number: height.slot_number,
            })
                .into(),
        };

        let response: QueryConnectionResponse = rollup
            .json_rpc_client()
            .request("ibc_connection", (request,))
            .await
            .map_err(Rollup::raise_error)?;

        Ok(response.conn_end)
    }
}

impl<Rollup, Counterparty, Encoding> ConnectionEndWithProofsQuerier<Rollup, Counterparty>
    for QueryConnectionEndOnSovereign
where
    Rollup: HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>
        + HasIbcChainTypes<Counterparty, Height = RollupHeight>
        + HasCommitmentProofType<CommitmentProof = SovereignCommitmentProof>
        + HasJsonRpcClient
        + HasEncoding<Encoding = Encoding>
        + CanRaiseError<Encoding::Error>
        + CanRaiseError<ClientError>,
    Rollup::JsonRpcClient: ClientT,
    Encoding: HasEncodedType<Encoded = Vec<u8>> + CanDecode<ViaBorsh, JellyfishMerkleProof>,
{
    async fn query_connection_end_with_proofs(
        rollup: &Rollup,
        connection_id: &Rollup::ConnectionId,
        query_height: &Rollup::Height,
    ) -> Result<(Rollup::ConnectionEnd, SovereignCommitmentProof), Rollup::Error> {
        let request = Request {
            connection_id: &connection_id.to_string(),
            query_height: &HeightParam {
                revision_number: 0,
                revision_height: query_height.slot_number,
            },
        };

        let response: QueryConnectionResponse = rollup
            .json_rpc_client()
            .request("ibc_connection", (request,))
            .await
            .map_err(Rollup::raise_error)?;

        let connection_end = response.conn_end;

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

        Ok((connection_end, commitment_proof))
    }
}

#[derive(Serialize)]
pub struct Request<'a> {
    pub connection_id: &'a str,
    pub query_height: &'a HeightParam,
}
