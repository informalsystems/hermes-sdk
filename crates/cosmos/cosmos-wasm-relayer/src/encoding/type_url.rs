use hermes_cosmos_chain_components::encoding::CosmosClientEncodingComponents;
use hermes_cosmos_chain_components::types::{TendermintClientState, TendermintConsensusState};
use hermes_prelude::*;
use hermes_wasm_encoding_components::components::WasmEncodingComponents;
use hermes_wasm_encoding_components::types::{
    WasmClientMessage, WasmClientState, WasmConsensusState,
};

pub struct WasmCosmosTypeUrlSchemas;

delegate_components! {
    WasmCosmosTypeUrlSchemas {
        [
            TendermintClientState,
            TendermintConsensusState,
        ]:
            CosmosClientEncodingComponents::Provider,
        [
            WasmClientState,
            WasmConsensusState,
            WasmClientMessage,
        ]:
            WasmEncodingComponents::Provider,

    }
}
