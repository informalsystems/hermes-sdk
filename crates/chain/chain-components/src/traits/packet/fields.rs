use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::ibc::packet::{
    HasIncomingPacketType, HasOutgoingPacketType,
};
use hermes_chain_type_components::traits::types::ibc::port_id::HasPortIdType;
use hermes_chain_type_components::traits::types::ibc::sequence::HasSequenceType;
use hermes_chain_type_components::traits::types::timeout::HasTimeoutType;

#[derive_component(OutgoingPacketFieldsReaderComponent, OutgoingPacketFieldsReader<Chain>)]
pub trait CanReadOutgoingPacketFields<Counterparty>:
    HasOutgoingPacketType<Counterparty>
    + HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasSequenceType<Counterparty>
where
    Counterparty: HasHeightType + HasTimeoutType + HasChannelIdType<Self> + HasPortIdType<Self>,
{
    fn outgoing_packet_src_channel_id(packet: &Self::OutgoingPacket) -> &Self::ChannelId;

    fn outgoing_packet_dst_channel_id(packet: &Self::OutgoingPacket) -> &Counterparty::ChannelId;

    fn outgoing_packet_src_port(packet: &Self::OutgoingPacket) -> &Self::PortId;

    fn outgoing_packet_dst_port(packet: &Self::OutgoingPacket) -> &Counterparty::PortId;

    fn outgoing_packet_sequence(packet: &Self::OutgoingPacket) -> &Self::Sequence;

    fn outgoing_packet_timeout_height(
        packet: &Self::OutgoingPacket,
    ) -> Option<Counterparty::Height>;

    fn outgoing_packet_timeout_timestamp(
        packet: &Self::OutgoingPacket,
    ) -> Option<Counterparty::Timeout>;
}

pub trait CanReadIncomingPacketFields<Counterparty>:
    HasHeightType
    + HasTimeoutType
    + HasIncomingPacketType<Counterparty>
    + HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
where
    Counterparty: HasChannelIdType<Self> + HasPortIdType<Self> + HasSequenceType<Self>,
{
    fn incoming_packet_src_channel_id(packet: &Self::IncomingPacket) -> &Counterparty::ChannelId;

    fn incoming_packet_dst_channel_id(packet: &Self::IncomingPacket) -> &Self::ChannelId;

    fn incoming_packet_src_port(packet: &Self::IncomingPacket) -> &Counterparty::PortId;

    fn incoming_packet_dst_port(packet: &Self::IncomingPacket) -> &Self::PortId;

    fn incoming_packet_sequence(packet: &Self::IncomingPacket) -> &Counterparty::Sequence;

    fn incoming_packet_timeout_height(packet: &Self::IncomingPacket) -> Option<Self::Height>;

    fn incoming_packet_timeout_timestamp(packet: &Self::IncomingPacket) -> Option<Self::Timeout>;
}

impl<Chain, Counterparty> CanReadIncomingPacketFields<Counterparty> for Chain
where
    Counterparty: CanReadOutgoingPacketFields<Chain>,
    Chain: HasHeightType
        + HasTimeoutType
        + HasChannelIdType<Counterparty>
        + HasPortIdType<Counterparty>,
{
    fn incoming_packet_src_channel_id(packet: &Self::IncomingPacket) -> &Counterparty::ChannelId {
        Counterparty::outgoing_packet_src_channel_id(packet)
    }

    fn incoming_packet_dst_channel_id(packet: &Self::IncomingPacket) -> &Self::ChannelId {
        Counterparty::outgoing_packet_dst_channel_id(packet)
    }

    fn incoming_packet_src_port(packet: &Self::IncomingPacket) -> &Counterparty::PortId {
        Counterparty::outgoing_packet_src_port(packet)
    }

    fn incoming_packet_dst_port(packet: &Self::IncomingPacket) -> &Self::PortId {
        Counterparty::outgoing_packet_dst_port(packet)
    }

    fn incoming_packet_sequence(packet: &Self::IncomingPacket) -> &Counterparty::Sequence {
        Counterparty::outgoing_packet_sequence(packet)
    }

    fn incoming_packet_timeout_height(packet: &Self::IncomingPacket) -> Option<Self::Height> {
        Counterparty::outgoing_packet_timeout_height(packet)
    }

    fn incoming_packet_timeout_timestamp(packet: &Self::IncomingPacket) -> Option<Self::Timeout> {
        Counterparty::outgoing_packet_timeout_timestamp(packet)
    }
}
