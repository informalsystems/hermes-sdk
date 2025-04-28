use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
use hermes_prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::packet::HasPacketType;

#[cgp_component {
  provider: PacketHeaderGetter,
  context: Chain,
}]
pub trait HasPacketHeader<Counterparty>:
    HasPacketType<Counterparty> + HasPacketHeaderType<Counterparty>
{
    fn packet_header(packet: &Self::Packet) -> &Self::PacketHeader;
}

#[cgp_provider(PacketHeaderGetterComponent)]
impl<Chain, Counterparty, Provider> PacketHeaderGetter<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: HasPacketType<Counterparty> + HasPacketHeaderType<Counterparty>,
    Provider: FieldGetter<Chain::Packet, symbol!("header"), Value = Chain::PacketHeader>,
{
    fn packet_header(packet: &Chain::Packet) -> &Chain::PacketHeader {
        Provider::get_field(packet, PhantomData)
    }
}
