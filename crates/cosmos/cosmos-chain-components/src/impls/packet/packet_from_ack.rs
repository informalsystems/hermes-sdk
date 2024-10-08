use hermes_relayer_components::chain::traits::packet::from_write_ack::PacketFromWriteAckBuilder;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use ibc_relayer_types::core::ics04_channel::events::WriteAcknowledgement;
use ibc_relayer_types::core::ics04_channel::packet::Packet;

pub struct BuildCosmosPacketFromWriteAck;

impl<Chain, Counterparty> PacketFromWriteAckBuilder<Chain, Counterparty>
    for BuildCosmosPacketFromWriteAck
where
    Chain: HasWriteAckEvent<Counterparty, WriteAckEvent = WriteAcknowledgement>,
    Counterparty: HasOutgoingPacketType<Chain, OutgoingPacket = Packet>,
{
    fn build_packet_from_write_ack_event(ack: &WriteAcknowledgement) -> &Packet {
        &ack.packet
    }
}
