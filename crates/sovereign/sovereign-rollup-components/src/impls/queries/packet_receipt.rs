use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::packet_receipt::PacketReceiptQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::packets::timeout::HasPacketReceiptType;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc_query::core::channel::QueryPacketReceiptResponse;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::ClientError;
use serde::Serialize;

use crate::traits::json_rpc_client::HasJsonRpcClient;
use crate::types::height::RollupHeight;
use crate::types::rpc::height::HeightParam;

pub struct QueryPacketReceiptFromSovereign;

impl<Rollup, Counterparty> PacketReceiptQuerier<Rollup, Counterparty>
    for QueryPacketReceiptFromSovereign
where
    Rollup: HasIbcChainTypes<Counterparty, Height = RollupHeight>
        + HasPacketReceiptType<Counterparty, PacketReceipt = bool>
        + HasCommitmentProofType<CommitmentProof = Vec<u8>>
        + HasJsonRpcClient
        + CanRaiseError<ClientError>,
    Counterparty: HasIbcChainTypes<Rollup, Sequence = Sequence>,
    Rollup::JsonRpcClient: ClientT,
{
    async fn query_packet_receipt(
        rollup: &Rollup,
        channel_id: &Rollup::ChannelId,
        port_id: &Rollup::PortId,
        sequence: &Counterparty::Sequence,
        height: &Rollup::Height,
    ) -> Result<(Rollup::PacketReceipt, Rollup::CommitmentProof), Rollup::Error> {
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

        let response: QueryPacketReceiptResponse = rollup
            .json_rpc_client()
            .request("ibc_packetReceipt", (request,))
            .await
            .map_err(Rollup::raise_error)?;

        Ok((response.received, response.proof))
    }
}

#[derive(Serialize)]
pub struct Request<'a> {
    pub port_id: &'a str,
    pub channel_id: &'a str,
    pub sequence: &'a Sequence,
    pub query_height: &'a HeightParam,
}
