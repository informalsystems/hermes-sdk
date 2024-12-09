use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
use cgp::prelude::*;

use crate::traits::types::payload::header::HasPayloadHeaderType;
use crate::traits::types::payload::payload::HasPayloadType;

#[cgp_component {
  provider: PayloadHeaderGetter,
  context: Chain,
}]
pub trait HasPayloadHeader<Counterparty>:
    HasPayloadType<Counterparty> + HasPayloadHeaderType<Counterparty>
{
    fn payload_header(payload: &Self::Payload) -> &Self::PayloadHeader;
}

impl<Chain, Counterparty, Provider> PayloadHeaderGetter<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: HasPayloadType<Counterparty> + HasPayloadHeaderType<Counterparty>,
    Provider: FieldGetter<Chain::Payload, symbol!("header"), Value = Chain::PayloadHeader>,
{
    fn payload_header(payload: &Chain::Payload) -> &Chain::PayloadHeader {
        Provider::get_field(payload, PhantomData)
    }
}
