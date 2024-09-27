use hermes_relayer_components::chain::traits::packet::fields::PacketFieldsReader;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use ibc_relayer_types::core::ics04_channel::packet::{Packet, Sequence};
use ibc_relayer_types::core::ics04_channel::timeout::TimeoutHeight;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use ibc_relayer_types::timestamp::Timestamp;
use ibc_relayer_types::Height;

pub struct CosmosPacketFieldReader;

impl<Chain, Counterparty> PacketFieldsReader<Chain, Counterparty> for CosmosPacketFieldReader
where
    Chain: HasIbcPacketTypes<Counterparty, IncomingPacket = Packet, OutgoingPacket = Packet>
        + HasIbcChainTypes<
            Counterparty,
            Timeout = Timestamp,
            ChannelId = ChannelId,
            PortId = PortId,
            Sequence = Sequence,
        >,
    Counterparty: HasIbcChainTypes<
        Chain,
        Timeout = Timestamp,
        ChannelId = ChannelId,
        PortId = PortId,
        Sequence = Sequence,
    >,
    Chain::Height: From<Height>,
    Counterparty::Height: From<Height>,
{
    fn incoming_packet_src_channel_id(packet: &Packet) -> &ChannelId {
        &packet.source_channel
    }

    fn incoming_packet_dst_channel_id(packet: &Packet) -> &ChannelId {
        &packet.destination_channel
    }

    fn incoming_packet_src_port(packet: &Packet) -> &PortId {
        &packet.source_port
    }

    fn incoming_packet_dst_port(packet: &Packet) -> &PortId {
        &packet.destination_port
    }

    fn incoming_packet_sequence(packet: &Packet) -> &Sequence {
        &packet.sequence
    }

    fn incoming_packet_timeout_height(packet: &Packet) -> Option<Chain::Height> {
        match &packet.timeout_height {
            TimeoutHeight::Never => None,
            TimeoutHeight::At(h) => Some((*h).into()),
        }
    }

    fn incoming_packet_timeout_timestamp(packet: &Packet) -> Option<Timestamp> {
        Some(packet.timeout_timestamp)
    }

    fn outgoing_packet_src_channel_id(packet: &Packet) -> &ChannelId {
        &packet.source_channel
    }

    fn outgoing_packet_dst_channel_id(packet: &Packet) -> &ChannelId {
        &packet.destination_channel
    }

    fn outgoing_packet_src_port(packet: &Packet) -> &PortId {
        &packet.source_port
    }

    fn outgoing_packet_dst_port(packet: &Packet) -> &PortId {
        &packet.destination_port
    }

    fn outgoing_packet_sequence(packet: &Packet) -> &Sequence {
        &packet.sequence
    }

    fn outgoing_packet_timeout_height(packet: &Packet) -> Option<Counterparty::Height> {
        match &packet.timeout_height {
            TimeoutHeight::Never => None,
            TimeoutHeight::At(h) => Some((*h).into()),
        }
    }

    fn outgoing_packet_timeout_timestamp(packet: &Packet) -> Option<Timestamp> {
        Some(packet.timeout_timestamp)
    }
}
