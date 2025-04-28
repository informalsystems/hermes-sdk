use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
use hermes_chain_type_components::traits::HasAmountType;
use hermes_ibc_components::traits::types::payload::data::HasPayloadDataType;
use hermes_prelude::*;

#[cgp_component {
  provider: PayloadMintAmountGetter,
  context: Chain,
}]
pub trait HasPayloadMintAmount<Counterparty, App>:
    HasAmountType + HasPayloadDataType<Counterparty, App>
{
    // Note: the returned mint amount is from `Self`, because as incoming packet,
    // it would be used as `Counterparty::PayloadData` where `Self` = `Counterparty`.
    fn payload_mint_amount(payload_data: &Self::PayloadData) -> &Self::Amount;
}

#[cgp_provider(PayloadMintAmountGetterComponent)]
impl<Chain, Counterparty, App, Provider> PayloadMintAmountGetter<Chain, Counterparty, App>
    for WithProvider<Provider>
where
    Chain: HasAmountType + HasPayloadDataType<Counterparty, App>,
    Provider: FieldGetter<Chain::PayloadData, symbol!("amount"), Value = Chain::Amount>,
{
    fn payload_mint_amount(payload_data: &Chain::PayloadData) -> &Chain::Amount {
        Provider::get_field(payload_data, PhantomData)
    }
}
