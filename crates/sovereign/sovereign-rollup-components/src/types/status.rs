use ibc_relayer_types::timestamp::Timestamp;

use crate::types::height::RollupHeight;

#[derive(Debug)]
pub struct SovereignRollupStatus {
    pub height: RollupHeight,

    pub timestamp: Timestamp,
}
