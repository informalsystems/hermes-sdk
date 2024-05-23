use core::fmt::{Debug, Display};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RollupHeight {
    pub slot_number: u64,
}

impl Display for RollupHeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
