use cgp_core::HasErrorType;

use crate::chain::traits::components::packet_fields_reader::CanReadPacketFields;
use crate::chain::types::aliases::{ChannelIdOf, HeightOf, PortIdOf, SequenceOf, TimestampOf};
use crate::relay::traits::chains::HasRelayChains;

pub trait HasRelayPacketFields: HasRelayChains {
    /**
        The source port of a packet, which is a port ID on the source chain
        that corresponds to the destination chain.
    */
    fn packet_src_port(packet: &Self::Packet) -> &PortIdOf<Self::SrcChain, Self::DstChain>;

    /**
        The source channel ID of a packet, which is a channel ID on the source chain
        that corresponds to the destination chain.
    */
    fn packet_src_channel_id(packet: &Self::Packet)
        -> &ChannelIdOf<Self::SrcChain, Self::DstChain>;

    /**
        The destination port of a packet, which is a port ID on the destination chain
        that corresponds to the source chain.
    */
    fn packet_dst_port(packet: &Self::Packet) -> &PortIdOf<Self::DstChain, Self::SrcChain>;

    /**
        The destination channel ID of a packet, which is a channel ID on the destination chain
        that corresponds to the source chain.
    */
    fn packet_dst_channel_id(packet: &Self::Packet)
        -> &ChannelIdOf<Self::DstChain, Self::SrcChain>;

    /**
        The sequence a packet, which is a sequence stored on the source chain
        that corresponds to the destination chain.
    */
    fn packet_sequence(packet: &Self::Packet) -> &SequenceOf<Self::SrcChain, Self::DstChain>;

    /**
        The optional timeout height of a packet, which is a height on the destination chain.
    */
    fn packet_timeout_height(packet: &Self::Packet) -> Option<&HeightOf<Self::DstChain>>;

    /**
        The timeout timestamp of a packet, which is a timestamp on the destination chain.
    */
    fn packet_timeout_timestamp(packet: &Self::Packet) -> &TimestampOf<Self::DstChain>;
}

impl<Relay, SrcChain, DstChain, Packet> HasRelayPacketFields for Relay
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain, Packet = Packet>,
    SrcChain: CanReadPacketFields<DstChain, OutgoingPacket = Packet> + HasErrorType,
    DstChain: CanReadPacketFields<SrcChain, IncomingPacket = Packet> + HasErrorType,
{
    fn packet_src_port(packet: &Self::Packet) -> &PortIdOf<SrcChain, DstChain> {
        SrcChain::outgoing_packet_src_port(packet)
    }

    fn packet_src_channel_id(packet: &Self::Packet) -> &ChannelIdOf<SrcChain, DstChain> {
        SrcChain::outgoing_packet_src_channel_id(packet)
    }

    fn packet_dst_port(packet: &Self::Packet) -> &PortIdOf<DstChain, SrcChain> {
        SrcChain::outgoing_packet_dst_port(packet)
    }

    fn packet_dst_channel_id(packet: &Self::Packet) -> &ChannelIdOf<DstChain, SrcChain> {
        SrcChain::outgoing_packet_dst_channel_id(packet)
    }

    fn packet_sequence(packet: &Self::Packet) -> &SequenceOf<SrcChain, DstChain> {
        SrcChain::outgoing_packet_sequence(packet)
    }

    fn packet_timeout_height(packet: &Self::Packet) -> Option<&HeightOf<DstChain>> {
        SrcChain::outgoing_packet_timeout_height(packet)
    }

    fn packet_timeout_timestamp(packet: &Self::Packet) -> &TimestampOf<DstChain> {
        SrcChain::outgoing_packet_timeout_timestamp(packet)
    }
}
