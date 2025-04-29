use hermes_cosmos_core::protobuf_encoding_components::types::strategy::ViaProtobuf;
use hermes_cosmos_core::wasm_encoding_components::components::WasmEncodingComponents;
use hermes_cosmos_core::wasm_encoding_components::types::{
    WasmClientMessage, WasmClientState, WasmConsensusState,
};
use hermes_prelude::*;
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
