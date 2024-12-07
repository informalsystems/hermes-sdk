use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
use cgp::prelude::*;

use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::payload::payload::HasPayloadType;

#[derive_component(PacketPayloadsGetterComponent, PacketPayloadsGetter<Chain>)]
pub trait HasPacketPayloads<Counterparty>:
    HasPacketType<Counterparty> + HasPayloadType<Counterparty>
{
    fn packet_payloads(packet: &Self::Packet) -> &[Self::Payload];
}

impl<Chain, Counterparty, Provider, Payloads> PacketPayloadsGetter<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: HasPacketType<Counterparty> + HasPayloadType<Counterparty>,
    Provider: FieldGetter<Chain::Packet, symbol!("payloads"), Value = Payloads>,
    Payloads: AsRef<[Chain::Payload]> + 'static,
{
    fn packet_payloads(packet: &Chain::Packet) -> &[Chain::Payload] {
        Provider::get_field(packet, PhantomData).as_ref()
    }
}
