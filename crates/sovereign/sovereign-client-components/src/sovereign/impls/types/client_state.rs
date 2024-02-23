use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::client_state::ProvideClientStateType;
use sov_celestia_client::types::client_state::ClientState as SovereignClientState;

pub struct ProvideSovereignClientState;

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty>
    for ProvideSovereignClientState
where
    Chain: Async,
{
    type ClientState = SovereignClientState;
}
