use core::time::Duration;
use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics03_connection::version::Version;
use ibc_relayer_types::core::ics23_commitment::commitment::CommitmentProofBytes;
use ibc_relayer_types::proofs::ConsensusProof;
use sov_celestia_client::types::client_state::SovTmClientState as SovereignClientState;

pub struct SovereignConnectionOpenInitPayload {
    pub commitment_prefix: Vec<u8>,
}

pub struct SovereignConnectionOpenTryPayload {
    pub commitment_prefix: Vec<u8>,
    pub client_state: SovereignClientState,
    pub versions: Vec<Version>,
    pub delay_period: Duration,
    pub update_height: Height,
    pub proof_init: Vec<u8>,
    pub proof_client: Vec<u8>,
    pub proof_consensus: Vec<u8>,
    pub proof_consensus_height: Height,
}

pub struct SovereignConnectionOpenAckPayload {
    pub client_state: SovereignClientState,
    pub version: Version,
    pub update_height: Height,
    pub proof_try: CommitmentProofBytes,
    pub proof_client: CommitmentProofBytes,
    pub proof_consensus: ConsensusProof,
}

pub struct SovereignConnectionOpenConfirmPayload {
    // TODO: fill in fields
}
