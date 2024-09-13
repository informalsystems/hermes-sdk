use cgp::prelude::*;
use hermes_cosmos_chain_components::encoding::components::CosmosEncodingComponents;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use ibc::core::client::types::Height;

use crate::types::client_state::{EncodeWasmClientState, WasmClientState};

pub struct WasmEncodeMutComponents;

delegate_components! {
    WasmEncodeMutComponents {
        [
            (ViaProtobuf, Height),
        ]: CosmosEncodingComponents,

        (ViaProtobuf, WasmClientState):
            EncodeWasmClientState,
    }
}
