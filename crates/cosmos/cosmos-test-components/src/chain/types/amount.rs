use core::fmt::{self, Display};

use serde::Serialize;

use crate::chain::types::Denom;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize)]
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
