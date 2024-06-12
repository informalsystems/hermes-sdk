use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::packet_acknowledgement::PacketAcknowledgementQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::packets::ack::HasAcknowledgementType;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc_query::core::channel::QueryPacketAcknowledgementResponse;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::ClientError;
use serde::Serialize;

use crate::traits::json_rpc_client::HasJsonRpcClient;
use crate::types::height::RollupHeight;
use crate::types::rpc::height::HeightParam;

pub struct QueryPacketAcknowledgementFromSovereign;

impl<Rollup, Counterparty> PacketAcknowledgementQuerier<Rollup, Counterparty>
    for QueryPacketAcknowledgementFromSovereign
where
    Rollup: HasIbcChainTypes<Counterparty, Height = RollupHeight>
        + HasAcknowledgementType<Counterparty, Acknowledgement = Vec<u8>>
        + HasCommitmentProofType<CommitmentProof = Vec<u8>>
        + HasJsonRpcClient
        + CanRaiseError<ClientError>,
    Counterparty: HasIbcChainTypes<Rollup, Sequence = Sequence>,
    Rollup::JsonRpcClient: ClientT,
{
    async fn query_packet_acknowledgement(
        rollup: &Rollup,
        channel_id: &Rollup::ChannelId,
        port_id: &Rollup::PortId,
        sequence: &Counterparty::Sequence,
        height: &Rollup::Height,
    ) -> Result<(Rollup::Acknowledgement, Rollup::CommitmentProof), Rollup::Error> {
        std::thread::sleep(std::time::Duration::from_secs(1));

        let request = Request {
            channel_id: &channel_id.to_string(),
            port_id: &port_id.to_string(),
            sequence,
            query_height: &(&RollupHeight {
                slot_number: height.slot_number + 1,
            })
                .into(),
        };

        let response: QueryPacketAcknowledgementResponse = rollup
            .json_rpc_client()
            .request("ibc_packetAcknowledgement", (request,))
            .await
            .map_err(Rollup::raise_error)?;

        Ok((Vec::from(response.acknowledgement.as_ref()), response.proof))
    }
}

#[derive(Serialize)]
pub struct Request<'a> {
    pub port_id: &'a str,
    pub channel_id: &'a str,
    pub sequence: &'a Sequence,
    pub query_height: &'a HeightParam,
}
