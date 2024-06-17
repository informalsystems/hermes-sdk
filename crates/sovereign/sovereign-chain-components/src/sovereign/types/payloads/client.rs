use hermes_sovereign_rollup_components::types::client_state::SovereignClientState;
use ibc::core::client::types::Height;
use ibc_relayer_types::clients::ics07_tendermint::header::Header;
use sov_celestia_client::types::client_state::TendermintClientParams;
use sov_celestia_client::types::consensus_state::SovTmConsensusState;
use sov_celestia_client::types::sovereign::SovereignClientParams;

pub struct SovereignCreateClientPayload {
    pub client_state: SovereignClientState,
    pub consensus_state: SovTmConsensusState,
    pub code_hash: Vec<u8>,
    pub latest_height: Height,
}

pub struct SovereignUpdateClientPayload {
    pub datachain_header: Vec<Header>,
    pub initial_state_height: Height,
    pub final_state_height: Height,
    pub final_user_hash: [u8; 32],
    pub final_kernel_hash: [u8; 32],
    pub final_root_hash: [u8; 32],
}

pub struct SovereignCreateClientOptions {
    pub tendermint_params_config: TendermintClientParams,
    pub sovereign_client_params: SovereignClientParams,
    pub code_hash: Vec<u8>,
}
