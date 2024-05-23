use cgp_core::prelude::*;
use hermes_protobuf_encoding_components::impl_type_url;

use crate::types::client_state::WasmClientState;
use crate::types::consensus_state::WasmConsensusState;

pub struct WasmTypeUrlSchemas;

delegate_components! {
    WasmTypeUrlSchemas {
        WasmClientState: WasmClientStateUrl,
        WasmConsensusState: WasmConsensusStateUrl,
    }
}

impl_type_url!(WasmClientStateUrl, "/ibc.lightclients.wasm.v1.ClientState");
impl_type_url!(
    WasmConsensusStateUrl,
    "/ibc.lightclients.wasm.v1.ConsensusState"
);
