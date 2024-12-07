use cgp::prelude::*;

use crate::traits::types::amount::HasAmountType;
use crate::traits::types::denom::HasDenomType;

#[cgp_component {
  name: AmountDenomGetterComponent,
  provider: AmountDenomGetter,
  context: Chain,
}]
pub trait HasAmountDenom: HasAmountType + HasDenomType {
    fn amount_denom(amount: &Self::Amount) -> &Self::Denom;
}
