use alloc::sync::Arc;

use async_trait::async_trait;
use cgp_core::CanRaiseError;
use eyre::eyre;
use ibc_relayer::chain::cosmos::query::packet_query;
use ibc_relayer::chain::requests::{Qualified, QueryHeight, QueryPacketEventDataRequest};
use ibc_relayer_components::chain::traits::components::send_packets_querier::SendPacketQuerier;
use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use ibc_relayer_types::core::ics04_channel::packet::{Packet, Sequence};
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use ibc_relayer_types::events::WithBlockDataType;
use ibc_relayer_types::Height;
use tendermint_rpc::{Client, Order};

use crate::methods::event::try_extract_send_packet_event;
use crate::traits::rpc_client::HasRpcClient;

pub struct QueryCosmosSendPacket;

#[async_trait]
impl<Chain, Counterparty> SendPacketQuerier<Chain, Counterparty> for QueryCosmosSendPacket
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Height = Height,
            ChannelId = ChannelId,
            PortId = PortId,
            Sequence = Sequence,
        > + HasIbcPacketTypes<Counterparty, OutgoingPacket = Packet>
        + HasRpcClient
        + CanRaiseError<eyre::Report>,
    Counterparty: HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>,
{
    async fn query_send_packet_from_sequence(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        counterparty_channel_id: &Counterparty::ChannelId,
        counterparty_port_id: &Counterparty::PortId,
        sequence: &Chain::Sequence,
        height: &Chain::Height,
    ) -> Result<Chain::OutgoingPacket, Chain::Error> {
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
            .map_err(|e| Chain::raise_error(e.into()))?;

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
            .filter_map(try_extract_send_packet_event)
            .map(|event| event.packet.clone())
            .collect();

        let send_packet = send_packets
            .first()
            .ok_or_else(|| Chain::raise_error(eyre!("missing send packet")))?;

        Ok(send_packet.clone())
    }
}
