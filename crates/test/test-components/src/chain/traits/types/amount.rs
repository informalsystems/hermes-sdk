use cgp::prelude::*;
use hermes_chain_type_components::traits::types::amount::HasAmountType;

use crate::chain::traits::types::denom::HasDenomType;

#[cgp_component {
    provider: AmountDenomGetter,
}]
pub trait HasAmountDenom: HasAmountType + HasDenomType {
    fn amount_denom(amount: &Self::Amount) -> &Self::Denom;
}

#[cgp_component {
  name: AmountMethodsComponent,
  provider: ProvideAmountMethods,
  context: Chain,
}]
pub trait HasAmountMethods: HasAmountType + HasAsyncErrorType {
    fn add_amount(
        current: &Self::Amount,
        amount: &Self::Amount,
    ) -> Result<Self::Amount, Self::Error>;

    fn subtract_amount(
        current: &Self::Amount,
        amount: &Self::Amount,
    ) -> Result<Self::Amount, Self::Error>;
}
