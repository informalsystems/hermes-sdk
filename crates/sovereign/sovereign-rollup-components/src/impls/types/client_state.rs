use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::client_state::ProvideClientStateType;

use crate::types::client::client_state::SovereignRollupClientState;

pub struct ProvideSovereignRollupClientState;

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty>
    for ProvideSovereignRollupClientState
where
    Chain: Async,
{
    type ClientState = SovereignRollupClientState;
}
