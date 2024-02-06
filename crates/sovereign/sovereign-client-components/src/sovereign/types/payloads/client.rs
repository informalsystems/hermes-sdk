use hermes_cosmos_client_components::types::payloads::client::CosmosUpdateClientPayload;
use ibc_core::client::types::Height;
use sov_celestia_client::types::client_state::ClientState as SovereignClientState;
use sov_celestia_client::types::consensus_state::ConsensusState as SovereignConsensusState;

pub struct SovereignCreateClientPayload {
    pub client_state: SovereignClientState,
    pub consensus_state: SovereignConsensusState,
    // TODO: Add rollup payloads
    pub code_hash: Vec<u8>,
    pub latest_height: Height,
}

pub struct SovereignUpdateClientPayload {
    pub celestia_payload: CosmosUpdateClientPayload,
    // TODO: Add rollup payloads
}
