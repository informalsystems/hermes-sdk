use alloc::sync::Arc;

use cgp::core::error::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::send_packets::SendPacketQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::HasSendPacketEvent;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use ibc_relayer::chain::cosmos::query::packet_query;
use ibc_relayer::chain::requests::{Qualified, QueryHeight, QueryPacketEventDataRequest};
use ibc_relayer_types::core::ics04_channel::events::SendPacket;
use ibc_relayer_types::core::ics04_channel::packet::{Packet, Sequence};
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use ibc_relayer_types::events::WithBlockDataType;
use ibc_relayer_types::Height;
use tendermint::abci::Event as AbciEvent;
use tendermint_rpc::{Client, Error as RpcError, Order};

use crate::traits::rpc_client::HasRpcClient;

pub struct QueryCosmosSendPacket;

impl<Chain, Counterparty> SendPacketQuerier<Chain, Counterparty> for QueryCosmosSendPacket
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Height = Height,
            ChannelId = ChannelId,
            PortId = PortId,
            Sequence = Sequence,
            Event = Arc<AbciEvent>,
        > + HasOutgoingPacketType<Counterparty, OutgoingPacket = Packet>
        + HasSendPacketEvent<Counterparty, SendPacketEvent = SendPacket>
        + HasRpcClient
        + CanRaiseError<RpcError>
        + CanRaiseError<&'static str>,
    Counterparty: HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>,
{
    async fn query_send_packet_from_sequence(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &PortId,
        counterparty_channel_id: &ChannelId,
        counterparty_port_id: &PortId,
        sequence: &Sequence,
        height: &Height,
    ) -> Result<Packet, Chain::Error> {
        // The unreceived packet are queried from the source chain, so the destination
        // channel id and port id are the counterparty channel id and counterparty port id.
        let request = QueryPacketEventDataRequest {
            event_id: WithBlockDataType::SendPacket,
            source_channel_id: channel_id.clone(),
            source_port_id: port_id.clone(),
            destination_channel_id: counterparty_channel_id.clone(),
            destination_port_id: counterparty_port_id.clone(),
            sequences: vec![*sequence],
            height: Qualified::SmallerEqual(QueryHeight::Specific(*height)),
        };

        let mut events = vec![];
        let query = packet_query(&request, *sequence);

        let response = chain
            .rpc_client()
            .tx_search(query, false, 1, 10, Order::Descending)
            .await
            .map_err(Chain::raise_error)?;

        for tx in response.txs.iter() {
            let mut event = tx
                .tx_result
                .events
                .iter()
                .map(|event| Arc::new(event.clone()))
                .collect();
            events.append(&mut event);
        }

        let send_packets: Vec<Packet> = events
            .iter()
            .filter_map(Chain::try_extract_send_packet_event)
            .map(|event| event.packet.clone())
            .collect();

        let send_packet = send_packets
            .into_iter()
            .next()
            .ok_or_else(|| Chain::raise_error("missing send packet"))?;

        Ok(send_packet)
    }
}
