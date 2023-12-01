use core::fmt::{Debug, Display};

use cgp_core::prelude::*;

use crate::traits::chain::types::denom::HasDenomType;

#[derive_component(AmountTypeComponent, AmountTypeProvider<Chain>)]
pub trait HasAmountType: HasDenomType {
    type Amount: Debug + Display + Eq + PartialOrd + Clone + Async;

    fn amount_denom(amount: &Self::Amount) -> &Self::Denom;
}
