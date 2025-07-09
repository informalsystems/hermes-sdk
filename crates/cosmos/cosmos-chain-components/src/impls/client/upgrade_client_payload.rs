use ibc::primitives::Signer;

use crate::types::{TendermintClientState, TendermintConsensusState};

#[derive(Clone, Debug)]
pub struct CosmosUpgradeClientPayload {
    pub client_state: TendermintClientState,
    pub consensus_state: TendermintConsensusState,
    pub proof_upgrade_client: Vec<u8>,
    pub proof_upgrade_consensus_state: Vec<u8>,
    pub signer: Signer,
}
