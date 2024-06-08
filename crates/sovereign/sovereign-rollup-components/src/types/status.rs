use ibc_relayer_types::timestamp::Timestamp;

use crate::types::height::RollupHeight;

#[derive(Debug)]
pub struct SovereignRollupStatus {
    // TODO: Add fields
    // Rollup height and time
    // DA height and time that corresponds to the rollup status
    pub height: RollupHeight,

    pub timestamp: Timestamp,

    pub root_hash: Vec<u8>,
    pub user_hash: Vec<u8>,
    pub kernel_hash: Vec<u8>,
}
