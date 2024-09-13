use cgp::prelude::*;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use hermes_wasm_client_components::impls::encoding::components::WasmEncodingComponents;
use hermes_wasm_client_components::types::client_state::WasmClientState;
use hermes_wasm_client_components::types::consensus_state::WasmConsensusState;
use ibc::core::client::types::Height;

pub struct WasmCosmosEncodeMutComponents;

delegate_components! {
    WasmCosmosEncodeMutComponents {
        [
            (ViaProtobuf, Height),
            (ViaProtobuf, WasmClientState),
            (ViaProtobuf, WasmConsensusState),
        ]: WasmEncodingComponents,
    }
}
