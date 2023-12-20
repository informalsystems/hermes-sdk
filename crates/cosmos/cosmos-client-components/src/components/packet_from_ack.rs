use hermes_relayer_components::chain::traits::components::packet_from_write_ack_builder::PacketFromWriteAckBuilder;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use ibc_relayer_types::core::ics04_channel::events::WriteAcknowledgement;
use ibc_relayer_types::core::ics04_channel::packet::Packet;

pub struct BuildCosmosPacketFromWriteAck;

impl<Chain, Counterparty> PacketFromWriteAckBuilder<Chain, Counterparty>
    for BuildCosmosPacketFromWriteAck
where
    Chain: HasWriteAckEvent<Counterparty, WriteAckEvent = WriteAcknowledgement>
        + HasIbcPacketTypes<Counterparty, IncomingPacket = Packet>,
{
    fn build_packet_from_write_ack_event(ack: &WriteAcknowledgement) -> &Packet {
        &ack.packet
    }
}
