use cgp::core::Async;
use hermes_relayer_components::chain::traits::types::client_state::ProvideClientStateType;
use hermes_wasm_encoding_components::types::client_state::WasmClientState;

pub struct ProvideWasmClientState;

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty> for ProvideWasmClientState
where
    Chain: Async,
{
    type ClientState = WasmClientState;
}
