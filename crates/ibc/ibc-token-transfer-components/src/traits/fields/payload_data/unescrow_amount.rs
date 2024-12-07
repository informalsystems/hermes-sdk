use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_ibc_components::traits::types::payload::data::HasPayloadDataType;

#[derive_component(PayloadUnescrowAmountGetterComponent, PayloadUnescrowAmountGetter<Chain>)]
pub trait HasPayloadUnescrowAmount<Counterparty, App>:
    HasPayloadDataType<Counterparty, App>
where
    Counterparty: HasAmountType,
{
    // Note: the returned unescrow amount is from `Counterparty`, because as incoming packet,
    // it would be used as `Counterparty::PayloadData` where `Counterparty::Counterparty` = `Self`.
    fn payload_unescrow_amount(payload_data: &Self::PayloadData) -> &Counterparty::Amount;
}

impl<Chain, Counterparty, App, Provider> PayloadUnescrowAmountGetter<Chain, Counterparty, App>
    for WithProvider<Provider>
where
    Chain: HasPayloadDataType<Counterparty, App>,
    Counterparty: HasAmountType,
    Provider: FieldGetter<Chain::PayloadData, symbol!("amount"), Value = Counterparty::Amount>,
{
    fn payload_unescrow_amount(payload_data: &Chain::PayloadData) -> &Counterparty::Amount {
        Provider::get_field(payload_data, PhantomData)
    }
}
