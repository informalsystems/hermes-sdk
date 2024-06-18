use cgp_core::CanRaiseError;
use hermes_encoding_components::traits::decoder::CanDecode;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_relayer_components::chain::traits::queries::packet_acknowledgement::PacketAcknowledgementQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::packets::ack::HasAcknowledgementType;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc_query::core::channel::QueryPacketAcknowledgementResponse;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::ClientError;
use serde::Serialize;

use crate::impls::borsh_encode::ViaBorsh;
use crate::traits::json_rpc_client::HasJsonRpcClient;
use crate::types::commitment_proof::{JellyfishMerkleProof, SovereignCommitmentProof};
use crate::types::height::RollupHeight;
use crate::types::rpc::height::HeightParam;

pub struct QueryPacketAcknowledgementFromSovereign;

impl<Rollup, Counterparty, Encoding> PacketAcknowledgementQuerier<Rollup, Counterparty>
    for QueryPacketAcknowledgementFromSovereign
where
    Rollup: HasIbcChainTypes<Counterparty, Height = RollupHeight>
        + HasAcknowledgementType<Counterparty, Acknowledgement = Vec<u8>>
        + HasCommitmentProofType<CommitmentProof = SovereignCommitmentProof>
        + HasJsonRpcClient
        + HasEncoding<Encoding = Encoding>
        + CanRaiseError<Encoding::Error>
        + CanRaiseError<ClientError>,
    Counterparty: HasIbcChainTypes<Rollup, Sequence = Sequence>,
    Rollup::JsonRpcClient: ClientT,
    Encoding: HasEncodedType<Encoded = Vec<u8>> + CanDecode<ViaBorsh, JellyfishMerkleProof>,
{
    async fn query_packet_acknowledgement(
        rollup: &Rollup,
        channel_id: &Rollup::ChannelId,
        port_id: &Rollup::PortId,
        sequence: &Counterparty::Sequence,
        height: &Rollup::Height,
    ) -> Result<(Rollup::Acknowledgement, Rollup::CommitmentProof), Rollup::Error> {
        let request = Request {
            channel_id: &channel_id.to_string(),
            port_id: &port_id.to_string(),
            sequence,
            query_height: &(&RollupHeight {
                slot_number: height.slot_number,
            })
                .into(),
        };

        let response: QueryPacketAcknowledgementResponse = rollup
            .json_rpc_client()
            .request("ibc_packetAcknowledgement", (request,))
            .await
            .map_err(Rollup::raise_error)?;

        let ack = Vec::from(response.acknowledgement.as_ref());

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

        Ok((ack, commitment_proof))
    }
}

#[derive(Serialize)]
pub struct Request<'a> {
    pub port_id: &'a str,
    pub channel_id: &'a str,
    pub sequence: &'a Sequence,
    pub query_height: &'a HeightParam,
}
