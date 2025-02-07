use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
use cgp::prelude::*;

use crate::traits::types::payload::data::HasPayloadDataType;
use crate::traits::types::payload::payload::HasPayloadType;

#[cgp_component {
  provider: PayloadDataGetter,
  context: Chain,
}]
pub trait HasPayloadData<Counterparty, App>:
    HasPayloadType<Counterparty> + HasPayloadDataType<Counterparty, App>
{
    fn payload_data(payload: &Self::Payload) -> &Self::PayloadData;
}

#[cgp_provider(PayloadDataGetterComponent)]
impl<Chain, Counterparty, App, Provider> PayloadDataGetter<Chain, Counterparty, App>
    for WithProvider<Provider>
where
    Chain: HasPayloadType<Counterparty> + HasPayloadDataType<Counterparty, App>,
    Provider: FieldGetter<Chain::Payload, symbol!("data"), Value = Chain::PayloadData>,
{
    fn payload_data(payload: &Chain::Payload) -> &Chain::PayloadData {
        Provider::get_field(payload, PhantomData)
    }
}
