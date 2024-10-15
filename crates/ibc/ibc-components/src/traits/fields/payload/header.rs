use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
use cgp::prelude::*;

use crate::traits::types::payload::header::HasPayloadHeaderType;
use crate::traits::types::payload::payload::HasPayloadType;

#[derive_component(PayloadHeaderGetterComponent, PayloadHeaderGetter<Chain>)]
pub trait HasPayloadHeader<Counterparty>:
    HasPayloadType<Counterparty> + HasPayloadHeaderType<Counterparty>
{
    fn payload_header(packet: &Self::Payload) -> &Self::PayloadHeader;
}

impl<Chain, Counterparty, Provider> PayloadHeaderGetter<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: HasPayloadType<Counterparty> + HasPayloadHeaderType<Counterparty>,
    Provider: FieldGetter<Chain::Payload, symbol!("header"), Field = Chain::PayloadHeader>,
{
    fn payload_header(packet: &Chain::Payload) -> &Chain::PayloadHeader {
        Provider::get_field(packet, PhantomData)
    }
}
