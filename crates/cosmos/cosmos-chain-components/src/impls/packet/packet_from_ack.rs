use cgp::prelude::HasAsyncErrorType;
use hermes_relayer_components::chain::traits::packet::from_write_ack::PacketFromWriteAckBuilder;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use ibc::core::channel::types::packet::Packet;

use crate::types::events::write_acknowledgment::WriteAckEvent;

pub struct BuildCosmosPacketFromWriteAck;

impl<Chain, Counterparty> PacketFromWriteAckBuilder<Chain, Counterparty>
    for BuildCosmosPacketFromWriteAck
where
    Chain: HasWriteAckEvent<Counterparty, WriteAckEvent = WriteAckEvent> + HasAsyncErrorType,
    Counterparty: HasOutgoingPacketType<Chain, OutgoingPacket = Packet>,
{
    async fn build_packet_from_write_ack_event(
        _chain: &Chain,
        ack: &WriteAckEvent,
    ) -> Result<Packet, Chain::Error> {
        Ok(ack.packet.clone())
    }
}
