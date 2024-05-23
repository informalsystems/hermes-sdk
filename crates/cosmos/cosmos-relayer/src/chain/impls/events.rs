use alloc::sync::Arc;

use hermes_cosmos_chain_components::methods::event::{
    try_extract_send_packet_event, try_extract_write_ack_event,
};
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::HasSendPacketEvent;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use ibc_relayer_types::core::ics04_channel::events::{SendPacket, WriteAcknowledgement};
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use tendermint::abci::Event as AbciEvent;

use crate::contexts::chain::CosmosChain;

impl<Counterparty> HasSendPacketEvent<Counterparty> for CosmosChain {
    type SendPacketEvent = SendPacket;

    fn try_extract_send_packet_event(event: &Arc<AbciEvent>) -> Option<SendPacket> {
        try_extract_send_packet_event(event)
    }

    fn extract_packet_from_send_packet_event(event: &SendPacket) -> Packet {
        event.packet.clone()
    }
}

impl<Counterparty> HasWriteAckEvent<Counterparty> for CosmosChain {
    type WriteAckEvent = WriteAcknowledgement;

    fn try_extract_write_ack_event(event: &Arc<AbciEvent>) -> Option<WriteAcknowledgement> {
        try_extract_write_ack_event(event)
    }
}
