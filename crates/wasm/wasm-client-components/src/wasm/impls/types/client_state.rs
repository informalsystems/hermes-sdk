use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::client_state::ProvideClientStateType;

use crate::wasm::types::messages::client::state::WasmClientState;

pub struct ProvideWasmClientState;

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty> for ProvideWasmClientState
where
    Chain: Async,
{
    type ClientState = WasmClientState;
}
