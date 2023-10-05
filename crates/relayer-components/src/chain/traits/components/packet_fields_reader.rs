use cgp_core::prelude::*;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::packet::HasIbcPacketTypes;

#[derive_component(PacketFieldsReaderComponent, PacketFieldsReader<Chain>)]
pub trait CanReadPacketFields<Counterparty>:
    HasIbcPacketTypes<Counterparty> + HasIbcChainTypes<Counterparty>
where
    Counterparty: HasIbcChainTypes<Self>,
{
    fn incoming_packet_src_channel_id(packet: &Self::IncomingPacket) -> &Counterparty::ChannelId;

    fn incoming_packet_dst_channel_id(packet: &Self::IncomingPacket) -> &Self::ChannelId;

    fn incoming_packet_src_port(packet: &Self::IncomingPacket) -> &Counterparty::PortId;

    fn incoming_packet_dst_port(packet: &Self::IncomingPacket) -> &Self::PortId;

    fn incoming_packet_sequence(packet: &Self::IncomingPacket) -> &Counterparty::Sequence;

    fn incoming_packet_timeout_height(packet: &Self::IncomingPacket) -> Option<&Self::Height>;

    fn incoming_packet_timeout_timestamp(packet: &Self::IncomingPacket) -> &Self::Timestamp;

    fn outgoing_packet_src_channel_id(packet: &Self::OutgoingPacket) -> &Self::ChannelId;

    fn outgoing_packet_dst_channel_id(packet: &Self::OutgoingPacket) -> &Counterparty::ChannelId;

    fn outgoing_packet_src_port(packet: &Self::OutgoingPacket) -> &Self::PortId;

    fn outgoing_packet_dst_port(packet: &Self::OutgoingPacket) -> &Counterparty::PortId;

    fn outgoing_packet_sequence(packet: &Self::OutgoingPacket) -> &Self::Sequence;

    fn outgoing_packet_timeout_height(
        packet: &Self::OutgoingPacket,
    ) -> Option<&Counterparty::Height>;

    fn outgoing_packet_timeout_timestamp(packet: &Self::OutgoingPacket)
        -> &Counterparty::Timestamp;
}
