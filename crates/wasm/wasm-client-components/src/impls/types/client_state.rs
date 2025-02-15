use cgp::prelude::*;
use hermes_cosmos_chain_components::components::cosmos_to_cosmos::ClientStateTypeComponent;
use hermes_relayer_components::chain::traits::types::client_state::ProvideClientStateType;
use hermes_wasm_encoding_components::types::client_state::WasmClientState;

pub struct ProvideWasmClientState;

#[cgp_provider(ClientStateTypeComponent)]
impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty> for ProvideWasmClientState
where
    Chain: Async,
{
    type ClientState = WasmClientState;
}
