use alloc::sync::Arc;

use cgp_core::error::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::ack_packets::AckPacketQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use ibc_relayer::chain::cosmos::query::packet_query;
use ibc_relayer::chain::requests::{Qualified, QueryHeight, QueryPacketEventDataRequest};
use ibc_relayer_types::core::ics04_channel::events::WriteAcknowledgement;
use ibc_relayer_types::core::ics04_channel::packet::{Packet, Sequence};
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use ibc_relayer_types::events::WithBlockDataType;
use ibc_relayer_types::Height;
use tendermint::abci::Event as AbciEvent;
use tendermint_rpc::{Client, Error as RpcError, Order};

use crate::traits::rpc_client::HasRpcClient;

pub struct QueryCosmosAckPacket;

impl<Chain, Counterparty> AckPacketQuerier<Chain, Counterparty> for QueryCosmosAckPacket
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Height = Height,
            ChannelId = ChannelId,
            PortId = PortId,
            Sequence = Sequence,
            Event = Arc<AbciEvent>,
        > + HasIbcPacketTypes<Counterparty, OutgoingPacket = Packet>
        + HasWriteAckEvent<Counterparty, WriteAckEvent = WriteAcknowledgement>
        + HasRpcClient
        + CanRaiseError<RpcError>
        + CanRaiseError<&'static str>,
    Counterparty:
        HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId> + HasWriteAckEvent<Chain>,
{
    async fn query_ack_packet_from_sequence(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &PortId,
        counterparty_channel_id: &ChannelId,
        counterparty_port_id: &PortId,
        sequence: &Sequence,
        height: &Height,
    ) -> Result<(Packet, WriteAcknowledgement), Chain::Error> {
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

        let write_acks: Vec<WriteAcknowledgement> = events
            .iter()
            .filter_map(Chain::try_extract_write_ack_event)
            .collect();

        let write_ack = write_acks
            .into_iter()
            .next()
            .ok_or_else(|| Chain::raise_error("missing ack packet"))?;

        Ok((write_ack.packet.clone(), write_ack))
    }
}
