use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::client_state::ProvideClientStateType;

use crate::sovereign::types::client_state::SovTmClientState;

pub struct ProvideSovereignClientState;

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty>
    for ProvideSovereignClientState
where
    Chain: Async,
{
    type ClientState = SovTmClientState;
}
