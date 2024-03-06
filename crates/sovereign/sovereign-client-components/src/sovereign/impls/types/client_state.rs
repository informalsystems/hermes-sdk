use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::client_state::ProvideClientStateType;
use hermes_relayer_components::encode::types::via::Via;
use hermes_wasm_client_components::types::client_state::WasmClientState;

use crate::sovereign::types::client_state::SovereignClientState;

pub struct ProvideSovereignClientState;

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty>
    for ProvideSovereignClientState
where
    Chain: Async,
{
    type ClientState = Via<WasmClientState, SovereignClientState>;
}
