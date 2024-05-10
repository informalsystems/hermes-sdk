use core::time::Duration;

use hermes_cosmos_chain_components::types::tendermint::TendermintClientState;
use ibc_relayer_types::core::ics03_connection::version::Version;
use ibc_relayer_types::Height;

use crate::types::sign_data::SolomachineTimestampedSignData;

pub struct SolomachineConnectionOpenInitPayload {
    pub commitment_prefix: Vec<u8>,
}

pub struct SolomachineConnectionOpenTryPayload {
    pub commitment_prefix: Vec<u8>,
    pub client_state: TendermintClientState,
    pub versions: Vec<Version>,
    pub delay_period: Duration,
    pub update_height: Height,
    pub proof_init: SolomachineTimestampedSignData,
    pub proof_client: SolomachineTimestampedSignData,
    pub proof_consensus: SolomachineTimestampedSignData,
}

pub struct SolomachineConnectionOpenAckPayload {
    pub client_state: TendermintClientState,
    pub version: Version,
    pub update_height: Height,
    pub proof_try: SolomachineTimestampedSignData,
    pub proof_client: SolomachineTimestampedSignData,
    pub proof_consensus: SolomachineTimestampedSignData,
}

pub struct SolomachineConnectionOpenConfirmPayload {
    pub update_height: Height,
    pub proof_ack: SolomachineTimestampedSignData,
}
