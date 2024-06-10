use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::packet_commitment::PacketCommitmentQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::packets::receive::HasPacketCommitmentType;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc_query::core::channel::QueryPacketCommitmentResponse;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::ClientError;
use serde::Serialize;

use crate::traits::json_rpc_client::HasJsonRpcClient;
use crate::types::height::RollupHeight;
use crate::types::rpc::height::HeightParam;

pub struct QueryPacketCommitmentFromSovereign;

impl<Rollup, Counterparty> PacketCommitmentQuerier<Rollup, Counterparty>
    for QueryPacketCommitmentFromSovereign
where
    Rollup: HasIbcChainTypes<Counterparty, Height = RollupHeight, Sequence = Sequence>
        + HasPacketCommitmentType<Counterparty, PacketCommitment = Vec<u8>>
        + HasCommitmentProofType<CommitmentProof = Vec<u8>>
        + HasJsonRpcClient
        + CanRaiseError<ClientError>,
    Rollup::JsonRpcClient: ClientT,
{
    async fn query_packet_commitment(
        rollup: &Rollup,
        channel_id: &Rollup::ChannelId,
        port_id: &Rollup::PortId,
        sequence: &Rollup::Sequence,
        height: &Rollup::Height,
    ) -> Result<(Rollup::PacketCommitment, Rollup::CommitmentProof), Rollup::Error> {
        let request = Request {
            channel_id: &channel_id.to_string(),
            port_id: &port_id.to_string(),
            sequence,
            query_height: &(&RollupHeight {
                slot_number: height.slot_number_for_proofs(),
            })
                .into(),
        };

        let response: QueryPacketCommitmentResponse = rollup
            .json_rpc_client()
            .request("ibc_packetCommitment", (request,))
            .await
            .map_err(Rollup::raise_error)?;

        Ok((response.packet_commitment.into_vec(), response.proof))
    }
}

#[derive(Serialize)]
pub struct Request<'a> {
    pub port_id: &'a str,
    pub channel_id: &'a str,
    pub sequence: &'a Sequence,
    pub query_height: &'a HeightParam,
}
