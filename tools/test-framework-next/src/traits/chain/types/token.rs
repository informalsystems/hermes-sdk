use cgp_core::Async;

use crate::traits::chain::types::amount::HasAmountType;
use crate::traits::chain::types::denom::HasDenomType;

pub trait HasTokenType {
    type Token: Async;
}

pub trait HasTokenFields: HasTokenType + HasAmountType + HasDenomType {
    fn token_amount(token: &Self::Token) -> &Self::Amount;

    fn token_denom(token: &Self::Token) -> &Self::Denom;
}
