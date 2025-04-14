use cgp::prelude::*;
use hermes_relayer_components::chain::traits::{ClientStateTypeComponent, ProvideClientStateType};
use hermes_wasm_encoding_components::types::client_state::WasmClientState;

pub struct ProvideWasmClientState;

#[cgp_provider(ClientStateTypeComponent)]
impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty> for ProvideWasmClientState
where
    Chain: Async,
{
    type ClientState = WasmClientState;
}
