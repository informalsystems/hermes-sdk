use hermes_cosmos_core::chain_components::encoding::CosmosClientEncodingComponents;
use hermes_cosmos_core::chain_components::types::{
    TendermintClientState, TendermintConsensusState,
};
use hermes_cosmos_core::wasm_encoding_components::components::WasmEncodingComponents;
use hermes_cosmos_core::wasm_encoding_components::types::{
    WasmClientMessage, WasmClientState, WasmConsensusState,
};
use hermes_prelude::*;

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
