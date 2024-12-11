use core::marker::PhantomData;

use cgp::core::Async;
use cgp::prelude::{HasErrorType, HasField};
use hermes_relayer_components::chain::traits::packet::fields::CanReadOutgoingPacketFields;
use hermes_relayer_components::chain::traits::packet::filter::{
    IncomingPacketFilter, OutgoingPacketFilter,
};
use hermes_relayer_components::chain::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use hermes_relayer_components::relay::traits::chains::{HasRelayChainTypes, HasRelayPacketType};
use hermes_relayer_components::relay::traits::packet_filter::RelayPacketFilter;
use ibc::core::host::types::identifiers::{ChannelId, PortId};

use crate::types::messages::packet::packet_filter::PacketFilterConfig;

pub struct FilterPacketWithConfig<Tag>(pub PhantomData<Tag>);

impl<Relay, Tag, SrcChain, DstChain> RelayPacketFilter<Relay> for FilterPacketWithConfig<Tag>
where
    Relay: HasRelayChainTypes<SrcChain = SrcChain, DstChain = DstChain>
        + HasRelayPacketType
        + HasField<Tag, Value = PacketFilterConfig>,
    SrcChain: CanReadOutgoingPacketFields<DstChain>
        + HasPortIdType<DstChain, PortId = PortId>
        + HasChannelIdType<DstChain, ChannelId = ChannelId>,
{
    async fn should_relay_packet(
        relay: &Relay,
        packet: &Relay::Packet,
    ) -> Result<bool, Relay::Error> {
        Ok(relay.get_field(PhantomData).is_allowed(
            SrcChain::outgoing_packet_src_port(packet),
            SrcChain::outgoing_packet_src_channel_id(packet),
        ))
    }
}

impl<Chain, Counterparty, Tag> OutgoingPacketFilter<Chain, Counterparty>
    for FilterPacketWithConfig<Tag>
where
    Chain: CanReadOutgoingPacketFields<Counterparty>
        + HasPortIdType<Counterparty, PortId = PortId>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasField<Tag, Value = PacketFilterConfig>
        + HasErrorType,
{
    async fn should_relay_outgoing_packet(
        chain: &Chain,
        packet: &Chain::OutgoingPacket,
    ) -> Result<bool, Chain::Error> {
        Ok(chain.get_field(PhantomData).is_allowed(
            Chain::outgoing_packet_src_port(packet),
            Chain::outgoing_packet_src_channel_id(packet),
        ))
    }
}

impl<Chain, Counterparty, Tag> IncomingPacketFilter<Chain, Counterparty>
    for FilterPacketWithConfig<Tag>
where
    Chain: Async
        + HasField<Tag, Value = PacketFilterConfig>
        + HasPortIdType<Counterparty, PortId = PortId>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasErrorType,
    Counterparty: CanReadOutgoingPacketFields<Chain>,
{
    async fn should_relay_incoming_packet(
        chain: &Chain,
        packet: &Counterparty::OutgoingPacket,
    ) -> Result<bool, Chain::Error> {
        Ok(chain.get_field(PhantomData).is_allowed(
            Counterparty::outgoing_packet_dst_port(packet),
            Counterparty::outgoing_packet_dst_channel_id(packet),
        ))
    }
}
