use core::fmt::{Debug, Display};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RollupHeight {
    pub slot_number: u128,
    // TODO: determine if we also need to include DA height here
}

impl Display for RollupHeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
