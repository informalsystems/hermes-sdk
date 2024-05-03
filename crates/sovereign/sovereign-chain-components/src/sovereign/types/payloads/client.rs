use ibc::clients::tendermint::types::Header;
use ibc::core::client::types::Height;
use sov_celestia_client::types::consensus_state::SovTmConsensusState;

use crate::sovereign::types::client_state::SovereignClientState;

pub struct SovereignCreateClientPayload {
    pub client_state: SovereignClientState,
    pub consensus_state: SovTmConsensusState,
    // TODO: Add rollup payloads
    pub code_hash: Vec<u8>,
    pub latest_height: Height,
}

pub struct SovereignUpdateClientPayload {
    pub datachain_header: Vec<Header>,
    pub initial_state_height: Height,
    pub final_state_height: Height,
    // TODO: Add rollup payloads
}

pub struct SovereignCreateClientOptions {
    pub chain_id: String,
    pub code_hash: Vec<u8>,
}
