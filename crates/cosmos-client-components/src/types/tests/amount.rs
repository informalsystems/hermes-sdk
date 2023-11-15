use core::cmp::Ordering;
use core::fmt::{self, Display};

use crate::types::tests::denom::Denom;

#[derive(Debug, PartialEq, Eq)]
pub struct Amount {
    pub quantity: u128,
    pub denom: Denom,
}

impl Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.quantity, self.denom)
    }
}

impl PartialOrd for Amount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.denom == other.denom {
            Some(self.quantity.cmp(&other.quantity))
        } else {
            None
        }
    }
}
