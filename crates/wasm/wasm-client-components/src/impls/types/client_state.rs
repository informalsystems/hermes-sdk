use hermes_core::relayer_components::chain::traits::{
    ClientStateTypeComponent, ProvideClientStateType,
};
use hermes_prelude::*;
use hermes_wasm_encoding_components::types::WasmClientState;

pub struct ProvideWasmClientState;

#[cgp_provider(ClientStateTypeComponent)]
impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty> for ProvideWasmClientState
where
    Chain: Async,
{
    type ClientState = WasmClientState;
}
