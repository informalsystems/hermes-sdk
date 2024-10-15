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
    fn payload_data(payload: &Self::Payload) -> &Self::PayloadData;
}

impl<Chain, Counterparty, App, Provider> PayloadDataGetter<Chain, Counterparty, App>
    for WithProvider<Provider>
where
    Chain: HasPayloadType<Counterparty> + HasPayloadDataType<Counterparty, App>,
    Provider: FieldGetter<Chain::Payload, symbol!("data"), Field = Chain::PayloadData>,
{
    fn payload_data(payload: &Chain::Payload) -> &Chain::PayloadData {
        Provider::get_field(payload, PhantomData)
    }
}
