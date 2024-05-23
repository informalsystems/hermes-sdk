use alloc::sync::Arc;

use hermes_cosmos_chain_components::methods::event::{
    try_extract_send_packet_event, try_extract_write_ack_event,
};
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::ProvideSendPacketEvent;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::ProvideWriteAckEvent;
use ibc_relayer_types::core::ics04_channel::events::{SendPacket, WriteAcknowledgement};
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use tendermint::abci::Event as AbciEvent;

use crate::chain::components::CosmosChainComponents;
use crate::contexts::chain::CosmosChain;

impl<Counterparty> ProvideSendPacketEvent<CosmosChain, Counterparty> for CosmosChainComponents {
    type SendPacketEvent = SendPacket;

    fn try_extract_send_packet_event(event: &Arc<AbciEvent>) -> Option<SendPacket> {
        try_extract_send_packet_event(event)
    }

    fn extract_packet_from_send_packet_event(event: &SendPacket) -> Packet {
        event.packet.clone()
    }
}

impl<Counterparty> ProvideWriteAckEvent<CosmosChain, Counterparty> for CosmosChainComponents {
    type WriteAckEvent = WriteAcknowledgement;

    fn try_extract_write_ack_event(event: &Arc<AbciEvent>) -> Option<WriteAcknowledgement> {
        try_extract_write_ack_event(event)
    }
}
