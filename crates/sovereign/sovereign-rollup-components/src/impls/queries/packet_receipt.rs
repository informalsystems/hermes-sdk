use cgp_core::CanRaiseError;
use hermes_encoding_components::traits::decoder::CanDecode;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_relayer_components::chain::traits::queries::packet_receipt::PacketReceiptQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::packets::timeout::HasPacketReceiptType;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc_query::core::channel::QueryPacketReceiptResponse;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::ClientError;
use serde::Serialize;

use crate::impls::borsh_encode::ViaBorsh;
use crate::traits::json_rpc_client::HasJsonRpcClient;
use crate::types::commitment_proof::{JellyfishMerkleProof, SovereignCommitmentProof};
use crate::types::height::RollupHeight;
use crate::types::rpc::height::HeightParam;

pub struct QueryPacketReceiptFromSovereign;

impl<Rollup, Counterparty, Encoding> PacketReceiptQuerier<Rollup, Counterparty>
    for QueryPacketReceiptFromSovereign
where
    Rollup: HasIbcChainTypes<Counterparty, Height = RollupHeight>
        + HasPacketReceiptType<Counterparty, PacketReceipt = bool>
        + HasCommitmentProofType<CommitmentProof = SovereignCommitmentProof>
        + HasJsonRpcClient
        + HasEncoding<Encoding = Encoding>
        + CanRaiseError<Encoding::Error>
        + CanRaiseError<ClientError>,
    Counterparty: HasIbcChainTypes<Rollup, Sequence = Sequence>,
    Rollup::JsonRpcClient: ClientT,
    Encoding: HasEncodedType<Encoded = Vec<u8>> + CanDecode<ViaBorsh, JellyfishMerkleProof>,
{
    async fn query_packet_receipt(
        rollup: &Rollup,
        channel_id: &Rollup::ChannelId,
        port_id: &Rollup::PortId,
        sequence: &Counterparty::Sequence,
        height: &Rollup::Height,
    ) -> Result<(Rollup::PacketReceipt, Rollup::CommitmentProof), Rollup::Error> {
        let request = Request {
            channel_id: &channel_id.to_string(),
            port_id: &port_id.to_string(),
            sequence,
            query_height: &(&RollupHeight {
                slot_number: height.slot_number,
            })
                .into(),
        };

        let response: QueryPacketReceiptResponse = rollup
            .json_rpc_client()
            .request("ibc_packetReceipt", (request,))
            .await
            .map_err(Rollup::raise_error)?;

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

        Ok((response.received, commitment_proof))
    }
}

#[derive(Serialize)]
pub struct Request<'a> {
    pub port_id: &'a str,
    pub channel_id: &'a str,
    pub sequence: &'a Sequence,
    pub query_height: &'a HeightParam,
}
