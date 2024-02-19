use crate::types::tendermint::TendermintClientState;
use cgp_core::prelude::Async;
use hermes_relayer_components::chain::traits::types::client_state::ProvideClientStateType;

pub struct ProvideTendermintClientState;

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty>
    for ProvideTendermintClientState
where
    Chain: Async,
{
    type ClientState = TendermintClientState;
}
