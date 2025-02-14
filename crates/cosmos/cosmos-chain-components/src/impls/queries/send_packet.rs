use alloc::sync::Arc;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_relayer_components::chain::traits::extract_data::CanExtractFromEvent;
use hermes_relayer_components::chain::traits::queries::send_packets::{
    SendPacketQuerier, SendPacketQuerierComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::HasSendPacketEvent;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use ibc::core::channel::types::packet::Packet;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::{ChannelId, PortId, Sequence};
use tendermint::abci::Event as AbciEvent;
use tendermint_rpc::query::Query;
use tendermint_rpc::{Client, Error as RpcError, Order};

use crate::traits::rpc_client::HasRpcClient;
use crate::types::events::send_packet::SendPacketEvent;

pub struct QueryCosmosSendPacket;

#[cgp_provider(SendPacketQuerierComponent)]
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
        + HasSendPacketEvent<Counterparty, SendPacketEvent = SendPacketEvent>
        + CanExtractFromEvent<SendPacketEvent>
        + HasRpcClient
        + CanRaiseAsyncError<RpcError>
        + CanRaiseAsyncError<&'static str>,
    Counterparty: HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>,
{
    async fn query_send_packet_from_sequence(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &PortId,
        counterparty_channel_id: &ChannelId,
        counterparty_port_id: &PortId,
        sequence: &Sequence,
    ) -> Result<Packet, Chain::Error> {
        // The unreceived packet are queried from the source chain, so the destination
        // channel id and port id are the counterparty channel id and counterparty port id.
        let query = Query::eq(
            format!("{}.packet_src_channel", "send_packet"),
            channel_id.to_string(),
        )
        .and_eq(
            format!("{}.packet_src_port", "send_packet"),
            port_id.to_string(),
        )
        .and_eq(
            format!("{}.packet_dst_channel", "send_packet"),
            counterparty_channel_id.to_string(),
        )
        .and_eq(
            format!("{}.packet_dst_port", "send_packet"),
            counterparty_port_id.to_string(),
        )
        .and_eq(
            format!("{}.packet_sequence", "send_packet"),
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

        let send_packets: Vec<Packet> = events
            .iter()
            .filter_map(|event| chain.try_extract_from_event(PhantomData, event))
            .map(|event| event.packet.clone())
            .collect();

        let send_packet = send_packets
            .into_iter()
            .next()
            .ok_or_else(|| Chain::raise_error("missing send packet"))?;

        Ok(send_packet)
    }
}
