use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
use cgp::prelude::*;

use crate::traits::types::payload::data::HasPayloadDataType;
use crate::traits::types::payload::payload::HasPayloadType;

#[derive_component(PayloadDataGetterComponent, PayloadDataGetter<Chain>)]
pub trait HasPayloadData<Counterparty, App>:
    HasPayloadType<Counterparty> + HasPayloadDataType<Counterparty, App>
{
    fn payload_header(packet: &Self::Payload) -> &Self::PayloadData;
}

impl<Chain, Counterparty, App, Provider> PayloadDataGetter<Chain, Counterparty, App>
    for WithProvider<Provider>
where
    Chain: HasPayloadType<Counterparty> + HasPayloadDataType<Counterparty, App>,
    Provider: FieldGetter<Chain::Payload, symbol!("header"), Field = Chain::PayloadData>,
{
    fn payload_header(packet: &Chain::Payload) -> &Chain::PayloadData {
        Provider::get_field(packet, PhantomData)
    }
}
