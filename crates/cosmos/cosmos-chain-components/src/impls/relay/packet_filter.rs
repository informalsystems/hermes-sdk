use core::marker::PhantomData;

use cgp::prelude::HasField;
use hermes_relayer_components::chain::traits::packet::fields::CanReadOutgoingPacketFields;
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
