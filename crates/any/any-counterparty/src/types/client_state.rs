use hermes_cosmos_chain_components::types::tendermint::TendermintClientState;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "client_type")]
pub enum AnyClientState {
    Tendermint(TendermintClientState),
    // WasmTendermint(WasmTendermintClientState),
}

impl AnyClientState {
    pub fn chain_id(&self) -> &ChainId {
        match self {
            AnyClientState::Tendermint(client_state) => &client_state.chain_id,
        }
    }
}
