use alloc::sync::Arc;
use core::marker::PhantomData;

use cgp::core::error::CanRaiseAsyncError;
use hermes_relayer_components::chain::traits::extract_data::CanExtractFromEvent;
use hermes_relayer_components::chain::traits::queries::ack_packets::AckPacketQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use ibc::core::channel::types::packet::Packet;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::{ChannelId, PortId, Sequence};
use tendermint::abci::Event as AbciEvent;
use tendermint_rpc::query::Query;
use tendermint_rpc::{Client, Error as RpcError, Order};

use crate::traits::rpc_client::HasRpcClient;
use crate::types::events::write_acknowledgment::WriteAckEvent;

pub struct QueryCosmosAckPacket;

impl<Chain, Counterparty> AckPacketQuerier<Chain, Counterparty> for QueryCosmosAckPacket
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Height = Height,
            ChannelId = ChannelId,
            PortId = PortId,
            Event = Arc<AbciEvent>,
        > + HasWriteAckEvent<Counterparty, WriteAckEvent = WriteAckEvent>
        + CanExtractFromEvent<WriteAckEvent>
        + HasRpcClient
        + CanRaiseAsyncError<RpcError>
        + CanRaiseAsyncError<&'static str>,
    Counterparty: HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId, Sequence = Sequence>
        + HasOutgoingPacketType<Chain, OutgoingPacket = Packet>
        + HasWriteAckEvent<Chain>,
{
    async fn query_ack_packet_from_sequence(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &PortId,
        counterparty_channel_id: &ChannelId,
        counterparty_port_id: &PortId,
        sequence: &Sequence,
        _height: &Height,
    ) -> Result<(Packet, WriteAckEvent), Chain::Error> {
        // The ack packet are queried from the destination chain, so the destination
        // channel id and port id are the counterparty channel id and counterparty port id.
        let query = Query::eq(
            format!("{}.packet_src_channel", "write_acknowledgement"),
            channel_id.to_string(),
        )
        .and_eq(
            format!("{}.packet_src_port", "write_acknowledgement"),
            port_id.to_string(),
        )
        .and_eq(
            format!("{}.packet_dst_channel", "write_acknowledgement"),
            counterparty_channel_id.to_string(),
        )
        .and_eq(
            format!("{}.packet_dst_port", "write_acknowledgement"),
            counterparty_port_id.to_string(),
        )
        .and_eq(
            format!("{}.packet_sequence", "write_acknowledgement"),
            sequence.to_string(),
        );

        let mut events = vec![];

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

        let write_acks: Vec<WriteAckEvent> = events
            .iter()
            .filter_map(|event| chain.try_extract_from_event(PhantomData, event))
            .collect();

        let write_ack = write_acks
            .into_iter()
            .next()
            .ok_or_else(|| Chain::raise_error("missing ack packet"))?;

        Ok((write_ack.packet.clone(), write_ack))
    }
}
