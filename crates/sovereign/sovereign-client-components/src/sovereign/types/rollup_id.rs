use core::fmt::{Debug, Display};

#[derive(Debug, Eq, PartialEq)]
pub struct RollupId(pub u64);

impl Display for RollupId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
