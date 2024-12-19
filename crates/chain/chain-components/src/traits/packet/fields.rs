use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
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
    provider: PacketFieldsReader,
    context: Chain,
}]
pub trait CanReadPacketFields<Counterparty>:
    HasOutgoingPacketType<Counterparty>
    + HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasSequenceType<Counterparty>
    + CanUseCounterparty<
        Counterparty,
        Counterparty: HasHeightType + HasTimeoutType + HasChannelIdType<Self> + HasPortIdType<Self>,
    >
{
    fn packet_src_channel_id(packet: &Self::OutgoingPacket) -> &Self::ChannelId;

    fn packet_dst_channel_id(packet: &Self::OutgoingPacket) -> &ChannelIdOf<Counterparty, Self>;

    fn packet_src_port(packet: &Self::OutgoingPacket) -> &Self::PortId;

    fn packet_dst_port(packet: &Self::OutgoingPacket) -> &PortIdOf<Counterparty, Self>;

    fn packet_sequence(packet: &Self::OutgoingPacket) -> &Self::Sequence;

    fn packet_timeout_height(packet: &Self::OutgoingPacket) -> Option<HeightOf<Counterparty>>;

    fn packet_timeout_timestamp(packet: &Self::OutgoingPacket) -> Option<TimeoutOf<Counterparty>>;
}

#[cgp_component {
    provider: PacketSrcChannelIdGetter,
    context: Chain,
}]
pub trait HasPacketSrcChannelId<Counterparty>:
    HasOutgoingPacketType<Counterparty> + HasChannelIdType<Counterparty>
{
    fn packet_src_channel_id(packet: &Self::OutgoingPacket) -> Self::ChannelId;
}

#[cgp_component {
    provider: PacketSrcPortIdGetter,
    context: Chain,
}]
pub trait HasPacketSrcPortId<Counterparty>:
    HasOutgoingPacketType<Counterparty> + HasPortIdType<Counterparty>
{
    fn packet_src_port_id(packet: &Self::OutgoingPacket) -> Self::PortId;
}

#[cgp_component {
    provider: PacketDstChannelIdGetter,
    context: Chain,
}]
pub trait HasPacketDstChannelId<Counterparty>:
    Sized
    + HasOutgoingPacketType<Counterparty>
    + CanUseCounterparty<Counterparty, Counterparty: HasChannelIdType<Self>>
{
    fn packet_dst_channel_id(packet: &Self::OutgoingPacket) -> ChannelIdOf<Counterparty, Self>;
}

#[cgp_component {
    provider: PacketDstPortIdGetter,
    context: Chain,
}]
pub trait HasPacketDstPortId<Counterparty>:
    Sized
    + HasOutgoingPacketType<Counterparty>
    + CanUseCounterparty<Counterparty, Counterparty: HasPortIdType<Self>>
{
    fn packet_dst_port_id(packet: &Self::OutgoingPacket) -> PortIdOf<Counterparty, Self>;
}

#[cgp_component {
    provider: PacketSequenceGetter,
    context: Chain,
}]
pub trait HasPacketSequence<Counterparty>:
    HasOutgoingPacketType<Counterparty> + HasSequenceType<Counterparty>
{
    fn packet_sequence(packet: &Self::OutgoingPacket) -> Self::Sequence;
}

#[cgp_component {
    provider: PacketTimeoutHeightGetter,
    context: Chain,
}]
pub trait HasPacketTimeoutHeight<Counterparty>:
    Sized
    + HasOutgoingPacketType<Counterparty>
    + CanUseCounterparty<Counterparty, Counterparty: HasHeightType>
{
    fn packet_timeout_height(packet: &Self::OutgoingPacket) -> Option<HeightOf<Counterparty>>;
}

#[cgp_component {
    provider: PacketTimeoutTimestampGetter,
    context: Chain,
}]
pub trait HasPacketTimeoutTimestamp<Counterparty>:
    Sized
    + HasOutgoingPacketType<Counterparty>
    + CanUseCounterparty<Counterparty, Counterparty: HasTimeoutType>
{
    fn packet_timeout_timestamp(packet: &Self::OutgoingPacket) -> Option<TimeoutOf<Counterparty>>;
}

impl<Chain, Counterparty, Provider> PacketSrcChannelIdGetter<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: HasOutgoingPacketType<Counterparty> + HasChannelIdType<Counterparty, ChannelId: Clone>,
    Provider: FieldGetter<
        Chain::OutgoingPacket,
        PacketSrcChannelIdGetterComponent,
        Value = Chain::ChannelId,
    >,
{
    fn packet_src_channel_id(packet: &Chain::OutgoingPacket) -> Chain::ChannelId {
        Provider::get_field(packet, PhantomData).clone()
    }
}

impl<Chain, Counterparty, Provider> PacketSrcPortIdGetter<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: HasOutgoingPacketType<Counterparty> + HasPortIdType<Counterparty, PortId: Clone>,
    Provider:
        FieldGetter<Chain::OutgoingPacket, PacketSrcPortIdGetterComponent, Value = Chain::PortId>,
{
    fn packet_src_port_id(packet: &Chain::OutgoingPacket) -> Chain::PortId {
        Provider::get_field(packet, PhantomData).clone()
    }
}

impl<Chain, Counterparty, Provider> PacketDstChannelIdGetter<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: HasOutgoingPacketType<Counterparty>,
    Counterparty: HasChannelIdType<Chain, ChannelId: Clone>,
    Provider: FieldGetter<
        Chain::OutgoingPacket,
        PacketDstChannelIdGetterComponent,
        Value = Counterparty::ChannelId,
    >,
{
    fn packet_dst_channel_id(packet: &Chain::OutgoingPacket) -> Counterparty::ChannelId {
        Provider::get_field(packet, PhantomData).clone()
    }
}

impl<Chain, Counterparty, Provider> PacketDstPortIdGetter<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: HasOutgoingPacketType<Counterparty>,
    Counterparty: HasPortIdType<Chain, PortId: Clone>,
    Provider: FieldGetter<
        Chain::OutgoingPacket,
        PacketDstPortIdGetterComponent,
        Value = Counterparty::PortId,
    >,
{
    fn packet_dst_port_id(packet: &Chain::OutgoingPacket) -> Counterparty::PortId {
        Provider::get_field(packet, PhantomData).clone()
    }
}

impl<Chain, Counterparty, Provider> PacketSequenceGetter<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: HasOutgoingPacketType<Counterparty> + HasSequenceType<Counterparty, Sequence: Clone>,
    Provider:
        FieldGetter<Chain::OutgoingPacket, PacketSequenceGetterComponent, Value = Chain::Sequence>,
{
    fn packet_sequence(packet: &Chain::OutgoingPacket) -> Chain::Sequence {
        Provider::get_field(packet, PhantomData).clone()
    }
}

impl<Chain, Counterparty, Provider> PacketTimeoutHeightGetter<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: HasOutgoingPacketType<Counterparty>,
    Counterparty: HasHeightType<Height: Clone>,
    Provider: FieldGetter<
        Chain::OutgoingPacket,
        PacketDstChannelIdGetterComponent,
        Value = Option<Counterparty::Height>,
    >,
{
    fn packet_timeout_height(packet: &Chain::OutgoingPacket) -> Option<Counterparty::Height> {
        Provider::get_field(packet, PhantomData).clone()
    }
}

impl<Chain, Counterparty, Provider> PacketTimeoutTimestampGetter<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: HasOutgoingPacketType<Counterparty>,
    Counterparty: HasTimeoutType<Timeout: Clone>,
    Provider: FieldGetter<
        Chain::OutgoingPacket,
        PacketDstChannelIdGetterComponent,
        Value = Option<Counterparty::Timeout>,
    >,
{
    fn packet_timeout_timestamp(packet: &Chain::OutgoingPacket) -> Option<Counterparty::Timeout> {
        Provider::get_field(packet, PhantomData).clone()
    }
}
