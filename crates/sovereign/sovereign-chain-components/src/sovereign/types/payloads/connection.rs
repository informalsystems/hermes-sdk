use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics03_connection::version::Version;
use ibc_relayer_types::core::ics23_commitment::commitment::CommitmentProofBytes;
use ibc_relayer_types::proofs::ConsensusProof;

use crate::sovereign::types::client_state::SovereignClientState;

pub struct SovereignConnectionOpenInitPayload {
    pub commitment_prefix: Vec<u8>,
}

pub struct SovereignConnectionOpenTryPayload {
    // TODO: fill in fields
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
