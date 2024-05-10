use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics03_connection::version::Version;
use sov_celestia_client::types::client_state::SovTmClientState as SovereignClientState;

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
    pub proof_try: Vec<u8>,
    pub proof_client: Vec<u8>,
    pub proof_consensus: Vec<u8>,
    pub proof_consensus_height: Height,
}

pub struct SovereignConnectionOpenConfirmPayload {
    // TODO: fill in fields
}
