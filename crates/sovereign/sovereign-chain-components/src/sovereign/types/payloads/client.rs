use ibc::clients::tendermint::types::Header;
use ibc::core::client::types::Height;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use sov_celestia_client::types::client_state::TendermintClientParams;
use sov_celestia_client::types::consensus_state::SovTmConsensusState;
use sov_celestia_client::types::sovereign::SovereignClientParams;

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
    //pub chain_id: String,
    // pub genesis_height: Height, // TODO: Maybe use Height or RollupHeight depending on value queried
    pub tendermint_params_config: TendermintClientParams,
    pub sovereign_client_params: SovereignClientParams,
    pub code_hash: Vec<u8>,
}
