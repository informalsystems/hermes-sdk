use core::fmt::{Debug, Display};

use cgp_core::Async;

use crate::traits::chain::types::denom::HasDenomType;

pub trait HasAmountType: HasDenomType {
    type Amount: Debug + Display + Eq + PartialOrd + Async;

    fn amount_denom(amount: &Self::Amount) -> &Self::Denom;
}
