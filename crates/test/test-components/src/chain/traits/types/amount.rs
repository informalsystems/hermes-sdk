use core::fmt::{Debug, Display};

use cgp_core::prelude::*;

use crate::chain::traits::types::denom::HasDenomType;

pub type AmountOf<Chain> = <Chain as HasAmountType>::Amount;

#[derive_component(AmountTypeComponent, ProvideAmountType<Chain>)]
pub trait HasAmountType: HasDenomType {
    type Amount: Debug + Display + Eq + PartialOrd + Clone + Async;

    fn amount_denom(amount: &Self::Amount) -> &Self::Denom;
}
