use core::marker::PhantomData;

use cgp::core::Async;
use cgp::prelude::{HasErrorType, HasField};
use hermes_relayer_components::chain::traits::packet::fields::CanReadPacketFields;
use hermes_relayer_components::chain::traits::packet::filter::{
    IncomingPacketFilter, OutgoingPacketFilter,
};
use hermes_relayer_components::chain::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use ibc::core::host::types::identifiers::{ChannelId, PortId};

use crate::types::messages::packet::packet_filter::PacketFilterConfig;

pub struct FilterPacketWithConfig<Tag>(pub PhantomData<Tag>);

impl<Chain, Counterparty, Tag> OutgoingPacketFilter<Chain, Counterparty>
    for FilterPacketWithConfig<Tag>
where
    Chain: CanReadPacketFields<Counterparty>
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
            Chain::packet_src_port(packet),
            Chain::packet_src_channel_id(packet),
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
    Counterparty: CanReadPacketFields<Chain>,
{
    async fn should_relay_incoming_packet(
        chain: &Chain,
        packet: &Counterparty::OutgoingPacket,
    ) -> Result<bool, Chain::Error> {
        Ok(chain.get_field(PhantomData).is_allowed(
            Counterparty::packet_dst_port(packet),
            Counterparty::packet_dst_channel_id(packet),
        ))
    }
}
