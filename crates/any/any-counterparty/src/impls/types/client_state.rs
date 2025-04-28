use core::time::Duration;

use hermes_prelude::*;
use hermes_relayer_components::chain::traits::{
    ClientStateFieldsComponent, ClientStateFieldsGetter, ClientStateTypeComponent, HasChainIdType,
    HasClientStateType, HasHeightType, ProvideClientStateType,
};
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::ChainId;

use crate::types::AnyClientState;

pub struct ProvideAnyClientState;

#[cgp_provider(ClientStateTypeComponent)]
impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty> for ProvideAnyClientState
where
    Chain: Async,
{
    type ClientState = AnyClientState;
}

#[cgp_provider(ClientStateFieldsComponent)]
impl<Chain, Counterparty> ClientStateFieldsGetter<Chain, Counterparty> for ProvideAnyClientState
where
    Chain: HasChainIdType<ChainId = ChainId>
        + HasHeightType<Height = Height>
        + HasClientStateType<Counterparty, ClientState = AnyClientState>,
{
    fn client_state_latest_height(client_state: &AnyClientState) -> Height {
        match client_state {
            AnyClientState::Tendermint(cs) => cs.latest_height,
        }
    }

    fn client_state_is_frozen(client_state: &AnyClientState) -> bool {
        match client_state {
            AnyClientState::Tendermint(cs) => cs.frozen_height.is_some(),
        }
    }

    fn client_state_has_expired(client_state: &AnyClientState, elapsed: Duration) -> bool {
        match client_state {
            AnyClientState::Tendermint(cs) => cs.trusting_period < elapsed,
        }
    }

    fn client_state_chain_id(client_state: &AnyClientState) -> ChainId {
        match client_state {
            AnyClientState::Tendermint(cs) => cs.chain_id.clone(),
        }
    }
}
