use cgp_core::prelude::*;
use hermes_cosmos_chain_components::encoding::components::CosmosEncodingComponents;

use hermes_cosmos_chain_components::types::tendermint::{
    TendermintClientState, TendermintConsensusState,
};
use hermes_wasm_client_components::impls::encoding::components::WasmEncodingComponents;
use hermes_wasm_client_components::types::client_state::WasmClientState;
use hermes_wasm_client_components::types::consensus_state::WasmConsensusState;

pub struct WasmCosmosTypeUrlSchemas;

delegate_components! {
    WasmCosmosTypeUrlSchemas {
        [
            TendermintClientState,
            TendermintConsensusState,
        ]:
            CosmosEncodingComponents,
        [
            WasmClientState,
            WasmConsensusState,
        ]:
            WasmEncodingComponents,

    }
}
