use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::traits::types::packet::header::HasPacketHeaderType;

#[derive_component(PacketClientGetterComponent, PacketClientGetter<Chain>)]
pub trait HasPacketChannels<Counterparty>:
    HasPacketHeaderType<Counterparty> + HasChannelIdType<Counterparty>
where
    Counterparty: HasChannelIdType<Self>,
{
    fn packet_src_channel_id(packet_header: &Self::PacketHeader) -> &Self::ChannelId;

    fn packet_dst_channel_id(packet_header: &Self::PacketHeader) -> &Counterparty::ChannelId;
}
