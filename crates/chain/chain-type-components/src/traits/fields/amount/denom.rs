use hermes_prelude::*;

use crate::traits::{HasAmountType, HasDenomType};

#[cgp_component {
  provider: AmountDenomGetter,
  context: Chain,
}]
pub trait HasAmountDenom: HasAmountType + HasDenomType {
    fn amount_denom(amount: &Self::Amount) -> &Self::Denom;
}
