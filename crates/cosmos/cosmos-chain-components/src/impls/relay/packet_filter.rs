use core::marker::PhantomData;

use hermes_core::relayer_components::chain::traits::{
    HasChannelIdType, HasPacketDstChannelId, HasPacketDstPortId, HasPacketSrcChannelId,
    HasPacketSrcPortId, HasPortIdType, IncomingPacketFilter, IncomingPacketFilterComponent,
    OutgoingPacketFilter, OutgoingPacketFilterComponent,
};
use hermes_prelude::*;
use ibc::core::host::types::identifiers::{ChannelId, PortId};

use crate::types::PacketFilterConfig;

pub struct FilterPacketWithConfig<Tag>(pub PhantomData<Tag>);

#[cgp_provider(OutgoingPacketFilterComponent)]
impl<Chain, Counterparty, Tag> OutgoingPacketFilter<Chain, Counterparty>
    for FilterPacketWithConfig<Tag>
where
    Chain: HasPacketSrcChannelId<Counterparty>
        + HasPacketSrcPortId<Counterparty>
        + HasPortIdType<Counterparty, PortId = PortId>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasField<Tag, Value = PacketFilterConfig>
        + HasAsyncErrorType,
{
    async fn should_relay_outgoing_packet(
        chain: &Chain,
        packet: &Chain::OutgoingPacket,
    ) -> Result<bool, Chain::Error> {
        Ok(chain.get_field(PhantomData).is_allowed(
            &Chain::packet_src_port_id(packet),
            &Chain::packet_src_channel_id(packet),
        ))
    }
}

#[cgp_provider(IncomingPacketFilterComponent)]
impl<Chain, Counterparty, Tag> IncomingPacketFilter<Chain, Counterparty>
    for FilterPacketWithConfig<Tag>
where
    Chain: Async
        + HasField<Tag, Value = PacketFilterConfig>
        + HasPortIdType<Counterparty, PortId = PortId>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasAsyncErrorType,
    Counterparty: HasPacketDstChannelId<Chain> + HasPacketDstPortId<Chain>,
{
    async fn should_relay_incoming_packet(
        chain: &Chain,
        packet: &Counterparty::OutgoingPacket,
    ) -> Result<bool, Chain::Error> {
        Ok(chain.get_field(PhantomData).is_allowed(
            &Counterparty::packet_dst_port_id(packet),
            &Counterparty::packet_dst_channel_id(packet),
        ))
    }
}
