use cgp::prelude::*;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_ibc_components::traits::types::payload::data::HasPayloadDataType;

#[derive_component(PayloadMintAmountGetterComponent, PayloadMintAmountGetter<Chain>)]
pub trait HasPayloadMintAmount<Counterparty, App>:
    HasAmountType + HasPayloadDataType<Counterparty, App>
{
    // Note: the returned mint amount is from `Self`, because as incoming packet,
    // it would be used as `Counterparty::PayloadData` where `Self` = `Counterparty`.
    fn payload_mint_amount(payload_data: &Self::PayloadData) -> &Self::Amount;
}
