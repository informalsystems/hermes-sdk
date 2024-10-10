use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
use cgp::prelude::*;

use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::payload::data::HasPayloadDataType;
use crate::traits::types::payload::header::HasPayloadHeaderType;

#[derive_component(PacketPayloadsGetterComponent, PacketPayloadsGetter<Chain>)]
pub trait HasPacketPayloads<Counterparty, App>:
    HasPacketType<Counterparty>
    + HasPayloadHeaderType<Counterparty>
    + HasPayloadDataType<Counterparty, App>
{
    fn packet_payloads(packet: &Self::Packet) -> &[(Self::PayloadHeader, Self::PayloadData)];
}

impl<Chain, Counterparty, App, Provider, Payloads> PacketPayloadsGetter<Chain, Counterparty, App>
    for WithProvider<Provider>
where
    Chain: HasPacketType<Counterparty>
        + HasPayloadHeaderType<Counterparty>
        + HasPayloadDataType<Counterparty, App>,
    Provider: FieldGetter<Chain::Packet, symbol!("payloads"), Field = Payloads>,
    Payloads: AsRef<[(Chain::PayloadHeader, Chain::PayloadData)]> + 'static,
{
    fn packet_payloads(packet: &Chain::Packet) -> &[(Chain::PayloadHeader, Chain::PayloadData)] {
        Provider::get_field(packet, PhantomData).as_ref()
    }
}
