use cgp::prelude::*;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use hermes_wasm_encoding_components::components::WasmEncodingComponents;
use hermes_wasm_encoding_components::types::{
    WasmClientMessage, WasmClientState, WasmConsensusState,
};
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
