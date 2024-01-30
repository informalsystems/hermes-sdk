use core::fmt::{Debug, Display};

use ibc_relayer_types::core::ics24_host::identifier::ChainId;

#[derive(Debug, Eq, PartialEq)]
pub struct RollupId {
    pub da_chain_id: ChainId,
    pub da_namespace: String,
}

impl Display for RollupId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
