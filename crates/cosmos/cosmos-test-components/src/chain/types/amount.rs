use core::cmp::Ordering;
use core::fmt::{self, Display};

use serde::Serialize;

use crate::chain::types::denom::Denom;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Amount {
    pub quantity: u128,
    pub denom: Denom,
}

impl Amount {
    pub fn new(quantity: u128, denom: Denom) -> Self {
        Self { quantity, denom }
    }
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
