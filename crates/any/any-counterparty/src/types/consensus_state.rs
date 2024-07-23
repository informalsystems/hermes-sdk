use hermes_cosmos_chain_components::types::tendermint::TendermintConsensusState;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "client_type")]
pub enum AnyConsensusState {
    Tendermint(TendermintConsensusState),
    // WasmTendermint(WasmTendermintConsensusState),
}
