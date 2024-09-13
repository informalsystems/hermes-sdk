use cgp::prelude::*;
use hermes_cosmos_chain_components::encoding::components::CosmosEncodingComponents;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use hermes_wasm_client_components::types::client_state::{EncodeWasmClientState, WasmClientState};
use ibc::core::client::types::Height;

pub struct WasmCosmosEncodeMutComponents;

delegate_components! {
    WasmCosmosEncodeMutComponents {
        [
            (ViaProtobuf, Height),
        ]: CosmosEncodingComponents,

        (ViaProtobuf, WasmClientState):
            EncodeWasmClientState,
    }
}
