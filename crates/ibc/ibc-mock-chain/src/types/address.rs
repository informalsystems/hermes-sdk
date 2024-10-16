use core::fmt::{Debug, Display};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MockAddress {
    UserA,
    UserB,
    UserC,
    RelayerA,
}

impl Display for MockAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self, f)
    }
}
