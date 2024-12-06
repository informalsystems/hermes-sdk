use core::marker::PhantomData;

use cgp::prelude::HasField;
use hermes_relayer_components::chain::traits::packet::fields::CanReadOutgoingPacketFields;
use hermes_relayer_components::chain::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use hermes_relayer_components::relay::traits::chains::{HasRelayChainTypes, PacketOf};
use hermes_relayer_components::relay::traits::packet_filter::PacketFilter;
use ibc_relayer::config::filter::PacketFilter as PacketFilterConfig;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};

pub struct FilterPacketWithConfig<Tag>(pub PhantomData<Tag>);

impl<Relay, Tag, SrcChain, DstChain> PacketFilter<Relay> for FilterPacketWithConfig<Tag>
where
    Relay: HasRelayChainTypes<SrcChain = SrcChain, DstChain = DstChain>
        + HasField<Tag, Field = PacketFilterConfig>,
    SrcChain: CanReadOutgoingPacketFields<DstChain>
        + HasPortIdType<DstChain, PortId = PortId>
        + HasChannelIdType<DstChain, ChannelId = ChannelId>,
{
    async fn should_relay_packet(
        relay: &Relay,
        packet: &PacketOf<Relay>,
    ) -> Result<bool, Relay::Error> {
        Ok(relay.get_field(PhantomData).channel_policy.is_allowed(
            SrcChain::outgoing_packet_src_port(packet),
            SrcChain::outgoing_packet_src_channel_id(packet),
        ))
    }
}
