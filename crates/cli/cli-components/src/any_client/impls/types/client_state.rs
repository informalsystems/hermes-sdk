use cgp_core::prelude::Async;
use hermes_relayer_components::chain::traits::types::client_state::ProvideClientStateType;

use crate::any_client::types::client_state::AnyClientState;

pub struct ProvideAnyClientState;

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty> for ProvideAnyClientState
where
    Chain: Async,
{
    type ClientState = AnyClientState;
}
