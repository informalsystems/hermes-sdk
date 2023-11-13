use crate::traits::chain::types::amount::HasAmountType;
use crate::traits::chain::types::denom::HasDenomType;
use crate::traits::chain::types::token::HasTokenType;

pub trait CanGenerateRandomTokenAmount: HasDenomType + HasTokenType {
    fn random_token_amount(denom: &Self::Denom, min: usize, max: usize) -> Self::Token;
}

pub trait HasTokenFields: HasTokenType + HasAmountType + HasDenomType {
    fn token_amount(token: &Self::Token) -> &Self::Amount;

    fn token_denom(token: &Self::Token) -> &Self::Denom;
}

pub trait HasTokenMethods: HasTokenFields {
    fn token_from_amount(denom: &Self::Denom, amount: &Self::Amount) -> Self::Token;

    fn add_token_amount(token: &Self::Token, amount: &Self::Amount) -> Self::Token;

    fn subtract_token_amount(token: &Self::Token, amount: &Self::Amount) -> Self::Token;
}
