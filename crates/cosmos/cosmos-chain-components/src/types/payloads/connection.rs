use core::time::Duration;

use ibc_relayer_types::core::ics03_connection::version::Version;
use ibc_relayer_types::core::ics23_commitment::commitment::CommitmentProofBytes;
use ibc_relayer_types::Height;
use prost_types::Any;

#[derive(Debug)]
pub struct CosmosConnectionOpenInitPayload {
    pub commitment_prefix: Vec<u8>,
}

#[derive(Debug)]
pub struct CosmosConnectionOpenTryPayload {
    pub commitment_prefix: Vec<u8>,
    pub client_state: Any,
    pub versions: Vec<Version>,
    pub delay_period: Duration,
    pub update_height: Height,
    pub proof_init: Vec<u8>,
    pub proof_client: Vec<u8>,
    pub proof_consensus: Vec<u8>,
    pub proof_consensus_height: Height,
}

#[derive(Debug)]
pub struct CosmosConnectionOpenAckPayload {
    pub client_state: Any,
    pub version: Version,
    pub update_height: Height,
    pub proof_try: Vec<u8>,
    pub proof_client: Vec<u8>,
    pub proof_consensus: Vec<u8>,
    pub proof_consensus_height: Height,
}

#[derive(Debug)]
pub struct CosmosConnectionOpenConfirmPayload {
    pub update_height: Height,
    pub proof_ack: CommitmentProofBytes,
}
