use cgp_core::prelude::*;
use hermes_wasm_client_components::impls::encoding::type_url::WasmClientStateUrl;
use hermes_wasm_client_components::types::client_state::WasmClientState;

pub struct SovereignTypeUrlSchemas;

delegate_components! {
    SovereignTypeUrlSchemas {
        WasmClientState: WasmClientStateUrl,
    }
}
