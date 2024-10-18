use cgp::prelude::*;

use crate::traits::types::amount::HasAmountType;
use crate::traits::types::denom::HasDenomType;

#[derive_component(AmountDenomGetterComponent, AmountDenomGetter<Chain>)]
pub trait HasAmountDenom: HasAmountType + HasDenomType {
    fn amount_denom(amount: &Self::Amount) -> &Self::Denom;
}
