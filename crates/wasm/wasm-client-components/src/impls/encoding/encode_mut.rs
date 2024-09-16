use cgp::prelude::*;
use hermes_cosmos_chain_components::encoding::components::CosmosClientEncodingComponents;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use ibc::core::client::types::Height;

use crate::types::client_state::{EncodeWasmClientState, WasmClientState};
use crate::types::consensus_state::{EncodeWasmConsensusState, WasmConsensusState};

pub struct WasmEncodeMutComponents;

delegate_components! {
    WasmEncodeMutComponents {
        [
            (ViaProtobuf, Height),
        ]: CosmosClientEncodingComponents,

        (ViaProtobuf, WasmClientState):
            EncodeWasmClientState,

        (ViaProtobuf, WasmConsensusState):
            EncodeWasmConsensusState,
    }
}
