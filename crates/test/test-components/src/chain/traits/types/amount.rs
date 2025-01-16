use core::fmt::{Debug, Display};

use cgp::prelude::*;

use crate::chain::traits::types::denom::HasDenomType;

pub type AmountOf<Chain> = <Chain as HasAmountType>::Amount;

#[cgp_component {
  name: AmountTypeComponent,
  provider: ProvideAmountType,
  context: Chain,
}]
pub trait HasAmountType: HasDenomType {
    type Amount: Debug + Display + Eq + PartialOrd + Clone + Async;

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
