use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::traits::types::packet::header::HasPacketHeaderType;

#[cgp_component {
  provider: PacketChannelIdGetter,
  context: Chain,
}]
pub trait HasPacketChannelIds<Counterparty>:
    HasPacketHeaderType<Counterparty> + HasChannelIdType<Counterparty>
where
    Counterparty: HasChannelIdType<Self>,
{
    fn packet_src_channel_id(packet_header: &Self::PacketHeader) -> &Self::ChannelId;

    fn packet_dst_channel_id(packet_header: &Self::PacketHeader) -> &Counterparty::ChannelId;
}

impl<Chain, Counterparty, Provider> PacketChannelIdGetter<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: HasPacketHeaderType<Counterparty> + HasChannelIdType<Counterparty>,
    Counterparty: HasChannelIdType<Chain>,
    Provider: FieldGetter<Chain::PacketHeader, symbol!("src_channel_id"), Value = Chain::ChannelId>
        + FieldGetter<Chain::PacketHeader, symbol!("dst_channel_id"), Value = Counterparty::ChannelId>,
{
    fn packet_src_channel_id(packet_header: &Chain::PacketHeader) -> &Chain::ChannelId {
        Provider::get_field(packet_header, PhantomData::<symbol!("src_channel_id")>)
    }

    fn packet_dst_channel_id(packet_header: &Chain::PacketHeader) -> &Counterparty::ChannelId {
        Provider::get_field(packet_header, PhantomData::<symbol!("dst_channel_id")>)
    }
}
