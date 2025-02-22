use cgp::prelude::*;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use hermes_wasm_encoding_components::components::WasmEncodingComponents;
use hermes_wasm_encoding_components::types::client_message::WasmClientMessage;
use hermes_wasm_encoding_components::types::client_state::WasmClientState;
use hermes_wasm_encoding_components::types::consensus_state::WasmConsensusState;
use ibc::core::client::types::Height;

pub struct WasmCosmosEncodeMutComponents;

delegate_components! {
    WasmCosmosEncodeMutComponents {
        [
            (ViaProtobuf, Height),
            (ViaProtobuf, WasmClientState),
            (ViaProtobuf, WasmConsensusState),
            (ViaProtobuf, WasmClientMessage),
        ]: WasmEncodingComponents::Provider,
    }
}
