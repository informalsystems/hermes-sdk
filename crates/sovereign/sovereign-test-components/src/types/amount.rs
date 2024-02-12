use core::cmp::Ordering;
use core::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SovereignAmount {
    pub quantity: u128,
    pub denom: String,
}

impl SovereignAmount {
    pub fn new(quantity: u128, denom: String) -> Self {
        Self { quantity, denom }
    }
}

impl Display for SovereignAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.quantity, self.denom)
    }
}

impl PartialOrd for SovereignAmount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.denom == other.denom {
            Some(self.quantity.cmp(&other.quantity))
        } else {
            None
        }
    }
}
