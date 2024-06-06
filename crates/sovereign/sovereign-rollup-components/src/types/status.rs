use ibc_relayer_types::timestamp::Timestamp;

use crate::types::height::RollupHeight;

#[derive(Debug)]
pub struct SovereignRollupStatus {
    // TODO: Add fields
    // Rollup height and time
    // DA height and time that corresponds to the rollup status
    pub height: RollupHeight,

    pub timestamp: Timestamp,

    pub hash: Vec<u8>,
    pub state_root: Vec<u8>,
}
