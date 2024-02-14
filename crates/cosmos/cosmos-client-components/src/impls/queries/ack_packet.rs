use alloc::sync::Arc;

use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use eyre::eyre;
use hermes_relayer_components::chain::traits::queries::ack_packets::AckPacketQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::chain::types::aliases::WriteAckEventOf;
use ibc_relayer::chain::cosmos::query::packet_query;
use ibc_relayer::chain::requests::{Qualified, QueryHeight, QueryPacketEventDataRequest};
use ibc_relayer_types::core::ics04_channel::events::WriteAcknowledgement;
use ibc_relayer_types::core::ics04_channel::packet::{Packet, Sequence};
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use ibc_relayer_types::events::WithBlockDataType;
use ibc_relayer_types::Height;
use tendermint_rpc::{Client, Order};

use crate::methods::event::try_extract_write_ack_event;
use crate::traits::rpc_client::HasRpcClient;

pub struct QueryCosmosAckPacket;

#[async_trait]
impl<Chain, Counterparty> AckPacketQuerier<Chain, Counterparty> for QueryCosmosAckPacket
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Height = Height,
            ChannelId = ChannelId,
            PortId = PortId,
            Sequence = Sequence,
        > + HasIbcPacketTypes<Counterparty, OutgoingPacket = Packet>
        + HasRpcClient
        + CanRaiseError<eyre::Report>
        + HasWriteAckEvent<Counterparty, WriteAckEvent = WriteAcknowledgement>,
    Counterparty:
        HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId> + HasWriteAckEvent<Chain>,
{
    async fn query_ack_packet_from_sequence(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        counterparty_channel_id: &Counterparty::ChannelId,
        counterparty_port_id: &Counterparty::PortId,
        sequence: &Chain::Sequence,
        height: &Chain::Height,
    ) -> Result<(Chain::OutgoingPacket, WriteAckEventOf<Chain, Counterparty>), Chain::Error> {
        // The ack packet are queried from the destination chain, so the destination
        // channel id and port id are the counterparty channel id and counterparty port id.
        let request = QueryPacketEventDataRequest {
            event_id: WithBlockDataType::WriteAck,
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

        let ack_packets: Vec<WriteAcknowledgement> = events
            .iter()
            .filter_map(try_extract_write_ack_event)
            .collect();

        let ack_packet = ack_packets
            .first()
            .map(|ack| ack.packet.clone())
            .ok_or_else(|| Chain::raise_error(eyre!("missing ack")))?;
        let ack = ack_packets
            .first()
            .ok_or_else(|| Chain::raise_error(eyre!("missing ack packet")))?
            .clone();

        Ok((ack_packet.clone(), ack))
    }
}
