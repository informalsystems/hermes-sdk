use cgp::prelude::*;
use hermes_chain_type_components::traits::types::counterparty::CanUseCounterparty;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::ibc::packet::HasOutgoingPacketType;
use hermes_chain_type_components::traits::types::ibc::port_id::HasPortIdType;
use hermes_chain_type_components::traits::types::ibc::sequence::HasSequenceType;
use hermes_chain_type_components::traits::types::timeout::{HasTimeoutType, TimeoutOf};

use crate::types::aliases::{ChannelIdOf, HeightOf, PortIdOf};

#[cgp_component {
  provider: OutgoingPacketFieldsReader,
  context: Chain,
}]
pub trait CanReadOutgoingPacketFields<Counterparty>:
    HasOutgoingPacketType<Counterparty>
    + HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasSequenceType<Counterparty>
    + CanUseCounterparty<
        Counterparty,
        Counterparty: HasHeightType + HasTimeoutType + HasChannelIdType<Self> + HasPortIdType<Self>,
    >
{
    fn outgoing_packet_src_channel_id(packet: &Self::OutgoingPacket) -> &Self::ChannelId;

    fn outgoing_packet_dst_channel_id(
        packet: &Self::OutgoingPacket,
    ) -> &ChannelIdOf<Counterparty, Self>;

    fn outgoing_packet_src_port(packet: &Self::OutgoingPacket) -> &Self::PortId;

    fn outgoing_packet_dst_port(packet: &Self::OutgoingPacket) -> &PortIdOf<Counterparty, Self>;

    fn outgoing_packet_sequence(packet: &Self::OutgoingPacket) -> &Self::Sequence;

    fn outgoing_packet_timeout_height(
        packet: &Self::OutgoingPacket,
    ) -> Option<HeightOf<Counterparty>>;

    fn outgoing_packet_timeout_timestamp(
        packet: &Self::OutgoingPacket,
    ) -> Option<TimeoutOf<Counterparty>>;
}

pub trait HasPacketSrcChannelId<Counterparty>:
    HasOutgoingPacketType<Counterparty> + HasChannelIdType<Counterparty>
{
    fn packet_src_channel_id(packet: &Self::OutgoingPacket) -> Self::ChannelId;
}

pub trait HasPacketSrcPortId<Counterparty>:
    HasOutgoingPacketType<Counterparty> + HasPortIdType<Counterparty>
{
    fn packet_src_port_id(packet: &Self::OutgoingPacket) -> Self::PortId;
}

pub trait HasPacketDstChannelId<Counterparty>:
    Sized
    + HasOutgoingPacketType<Counterparty>
    + CanUseCounterparty<Counterparty, Counterparty: HasChannelIdType<Self>>
{
    fn packet_dst_channel_id(packet: &Self::OutgoingPacket) -> ChannelIdOf<Counterparty, Self>;
}

pub trait HasPacketDstPortId<Counterparty>:
    Sized
    + HasOutgoingPacketType<Counterparty>
    + CanUseCounterparty<Counterparty, Counterparty: HasPortIdType<Self>>
{
    fn packet_dst_port_id(packet: &Self::OutgoingPacket) -> PortIdOf<Counterparty, Self>;
}

pub trait HasPacketTimeoutHeight<Counterparty>:
    Sized
    + HasOutgoingPacketType<Counterparty>
    + CanUseCounterparty<Counterparty, Counterparty: HasHeightType>
{
    fn outgoing_packet_timeout_height(
        packet: &Self::OutgoingPacket,
    ) -> Option<HeightOf<Counterparty>>;
}

pub trait HasPacketTimeoutTimestamp<Counterparty>:
    Sized
    + HasOutgoingPacketType<Counterparty>
    + CanUseCounterparty<Counterparty, Counterparty: HasTimeoutType>
{
    fn outgoing_packet_timeout_timestamp(
        packet: &Self::OutgoingPacket,
    ) -> Option<TimeoutOf<Counterparty>>;
}
