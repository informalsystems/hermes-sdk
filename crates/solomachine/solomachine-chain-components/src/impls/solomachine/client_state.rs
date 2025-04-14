use core::time::Duration;

use cgp::prelude::*;
use hermes_chain_type_components::traits::HasChainIdType;
use hermes_relayer_components::chain::traits::{
    ClientStateFieldsComponent, ClientStateFieldsGetter, ClientStateTypeComponent,
    HasClientStateType, HasHeightType, ProvideClientStateType,
};
use ibc::core::client::types::Height;

use crate::types::client_state::SolomachineClientState;

pub struct ProvideSolomachineClientState;

#[cgp_provider(ClientStateTypeComponent)]
impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty>
    for ProvideSolomachineClientState
where
    Chain: Async,
{
    type ClientState = SolomachineClientState;
}

// TODO: properly implement solomachine client state fields
#[cgp_provider(ClientStateFieldsComponent)]
impl<Chain, Counterparty> ClientStateFieldsGetter<Chain, Counterparty>
    for ProvideSolomachineClientState
where
    Chain: HasClientStateType<Counterparty, ClientState = SolomachineClientState>
        + HasHeightType<Height = Height>
        + HasChainIdType,
{
    fn client_state_latest_height(client_state: &SolomachineClientState) -> Height {
        Height::new(0, client_state.sequence).unwrap()
    }

    fn client_state_is_frozen(client_state: &SolomachineClientState) -> bool {
        client_state.is_frozen
    }

    fn client_state_has_expired(
        _client_state: &SolomachineClientState,
        _elapsed: Duration,
    ) -> bool {
        false
    }

    fn client_state_chain_id(_client_state: &SolomachineClientState) -> Chain::ChainId {
        // Solomachine client state doesn't contain Chain ID
        unimplemented!()
    }
}
