use std::time::Duration;

use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateFields;
use ibc_relayer_types::core::ics02_client::client_state::ClientState;

use super::client_state::AnyClientState;

pub struct AnyCounterparty;

impl HasClientStateFields<CosmosChain> for AnyCounterparty {
    fn client_state_chain_id(client_state: &Self::ClientState) -> &Self::ChainId {
        match client_state {
            AnyClientState::Tendermint(cs) => &cs.chain_id,
        }
    }

    fn client_state_latest_height(client_state: &Self::ClientState) -> &Self::Height {
        match client_state {
            AnyClientState::Tendermint(cs) => &cs.latest_height,
        }
    }

    fn client_state_is_frozen(client_state: &Self::ClientState) -> bool {
        match client_state {
            AnyClientState::Tendermint(cs) => cs.frozen_height.is_some(),
        }
    }

    fn client_state_has_expired(client_state: &Self::ClientState, elapsed: Duration) -> bool {
        match client_state {
            AnyClientState::Tendermint(cs) => cs.expired(elapsed),
        }
    }
}

impl HasClientStateFields<AnyCounterparty> for AnyCounterparty {
    fn client_state_chain_id(client_state: &Self::ClientState) -> &Self::ChainId {
        match client_state {
            AnyClientState::Tendermint(cs) => &cs.chain_id,
        }
    }

    fn client_state_latest_height(client_state: &Self::ClientState) -> &Self::Height {
        match client_state {
            AnyClientState::Tendermint(cs) => &cs.latest_height,
        }
    }

    fn client_state_is_frozen(client_state: &Self::ClientState) -> bool {
        match client_state {
            AnyClientState::Tendermint(cs) => cs.frozen_height.is_some(),
        }
    }

    fn client_state_has_expired(client_state: &Self::ClientState, elapsed: Duration) -> bool {
        match client_state {
            AnyClientState::Tendermint(cs) => cs.expired(elapsed),
        }
    }
}
