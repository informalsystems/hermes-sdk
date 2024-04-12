use cgp_core::prelude::*;
use hermes_protobuf_encoding_components::impl_type_url;
use hermes_wasm_client_components::impls::encoding::components::WasmEncodingComponents;
use hermes_wasm_client_components::types::client_state::WasmClientState;
use hermes_wasm_client_components::types::consensus_state::WasmConsensusState;

use crate::sovereign::types::client_state::SovereignClientState;

pub struct SovereignTypeUrlSchemas;

delegate_components! {
    SovereignTypeUrlSchemas {
        [
            WasmClientState,
            WasmConsensusState,
        ]:
            WasmEncodingComponents,
        SovereignClientState:
            SovereignClientStateUrl,
    }
}

impl_type_url!(
    SovereignClientStateUrl,
    "/ibc.lightclients.sovereign.tendermint.v1.ClientState"
);
