use hermes_cosmos_client_components::types::tendermint::TendermintClientState;
use ibc_relayer::client_state::AnyClientState as RelayerAnyClientState;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "client_type")]
pub enum AnyClientState {
    Tendermint(TendermintClientState),
    // Sovereign(SovereignClientState),
}

impl AnyClientState {
    pub fn chain_id(&self) -> &ChainId {
        match self {
            AnyClientState::Tendermint(client_state) => &client_state.chain_id,
        }
    }
}

impl From<RelayerAnyClientState> for AnyClientState {
    fn from(client_state: RelayerAnyClientState) -> Self {
        match client_state {
            RelayerAnyClientState::Tendermint(client_state) => {
                AnyClientState::Tendermint(client_state)
            }
        }
    }
}
