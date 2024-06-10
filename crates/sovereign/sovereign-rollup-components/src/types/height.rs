use core::fmt::{Debug, Display};

use ibc_relayer_types::Height;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RollupHeight {
    pub slot_number: u64,
}

impl RollupHeight {
    // FIXME: currently due to inconsistencies in Sovereign SDK, the slot number used
    // for constructing proofs are in fact +2 or the reported slot number.
    pub fn slot_number_for_proofs(&self) -> u64 {
        self.slot_number + 2
    }
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
