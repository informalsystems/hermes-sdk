use cgp_core::CanRaiseError;
use hermes_encoding_components::traits::decoder::CanDecode;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_relayer_components::chain::traits::queries::channel_end::{
    ChannelEndQuerier, ChannelEndWithProofsQuerier,
};
use hermes_relayer_components::chain::traits::types::channel::HasChannelEndType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc::core::channel::types::channel::ChannelEnd;
use ibc_query::core::channel::QueryChannelResponse;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::ClientError;
use serde::Serialize;

use crate::impls::borsh_encode::ViaBorsh;
use crate::traits::json_rpc_client::HasJsonRpcClient;
use crate::types::commitment_proof::{JellyfishMerkleProof, SovereignCommitmentProof};
use crate::types::height::RollupHeight;
use crate::types::rpc::height::HeightParam;

pub struct QueryChannelEndOnSovereign;

impl<Rollup, Counterparty> ChannelEndQuerier<Rollup, Counterparty> for QueryChannelEndOnSovereign
where
    Rollup: HasChannelEndType<Counterparty, ChannelEnd = ChannelEnd>
        + HasIbcChainTypes<
            Counterparty,
            Height = RollupHeight,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasJsonRpcClient
        + CanRaiseError<ClientError>,
    Rollup::JsonRpcClient: ClientT,
{
    async fn query_channel_end(
        rollup: &Rollup,
        channel_id: &Rollup::ChannelId,
        port_id: &Rollup::PortId,
        height: &Rollup::Height,
    ) -> Result<Rollup::ChannelEnd, Rollup::Error> {
        let request = Request {
            channel_id: channel_id.as_ref(),
            port_id: port_id.as_ref(),
            query_height: &(&RollupHeight {
                slot_number: height.slot_number,
            })
                .into(),
        };

        let response: QueryChannelResponse = rollup
            .json_rpc_client()
            .request("ibc_channel", (request,))
            .await
            .map_err(Rollup::raise_error)?;

        Ok(response.channel)
    }
}

impl<Rollup, Counterparty, Encoding> ChannelEndWithProofsQuerier<Rollup, Counterparty>
    for QueryChannelEndOnSovereign
where
    Rollup: HasChannelEndType<Counterparty, ChannelEnd = ChannelEnd>
        + HasIbcChainTypes<
            Counterparty,
            Height = RollupHeight,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasCommitmentProofType<CommitmentProof = SovereignCommitmentProof>
        + HasJsonRpcClient
        + HasEncoding<Encoding = Encoding>
        + CanRaiseError<Encoding::Error>
        + CanRaiseError<ClientError>,
    Rollup::JsonRpcClient: ClientT,
    Encoding: HasEncodedType<Encoded = Vec<u8>> + CanDecode<ViaBorsh, JellyfishMerkleProof>,
{
    async fn query_channel_end_with_proofs(
        rollup: &Rollup,
        channel_id: &Rollup::ChannelId,
        port_id: &Rollup::PortId,
        height: &Rollup::Height,
    ) -> Result<(Rollup::ChannelEnd, SovereignCommitmentProof), Rollup::Error> {
        let request = Request {
            channel_id: channel_id.as_ref(),
            port_id: port_id.as_ref(),
            query_height: &(&RollupHeight {
                slot_number: height.slot_number,
            })
                .into(),
        };

        let response: QueryChannelResponse = rollup
            .json_rpc_client()
            .request("ibc_channel", (request,))
            .await
            .map_err(Rollup::raise_error)?;

        let channel_end = response.channel;

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

        Ok((channel_end, commitment_proof))
    }
}

#[derive(Serialize)]
pub struct Request<'a> {
    pub channel_id: &'a str,
    pub port_id: &'a str,
    pub query_height: &'a HeightParam,
}
