use core::fmt::{Debug, Display};

use ibc_relayer_types::Height;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RollupHeight {
    pub slot_number: u64,
}

impl Display for RollupHeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl From<Height> for RollupHeight {
    fn from(height: Height) -> Self {
        Self {
            slot_number: height.revision_height(),
        }
    }
}
