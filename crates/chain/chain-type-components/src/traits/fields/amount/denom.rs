use cgp::prelude::*;

use crate::traits::types::amount::HasAmountType;
use crate::traits::types::denom::HasDenomType;

#[cgp_component {
  provider: AmountDenomGetter,
  context: Chain,
}]
pub trait HasAmountDenom: HasAmountType + HasDenomType {
    fn amount_denom(amount: &Self::Amount) -> &Self::Denom;
}
