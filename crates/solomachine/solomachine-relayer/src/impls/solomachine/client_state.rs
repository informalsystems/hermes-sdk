use core::time::Duration;

use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsGetter, HasClientStateType, ProvideClientStateType,
};
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use ibc_relayer_types::core::ics02_client::height::Height;

use crate::types::client_state::SolomachineClientState;

pub struct ProvideSolomachineClientState;

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty>
    for ProvideSolomachineClientState
where
    Chain: Async,
{
    type ClientState = SolomachineClientState;
}

// TODO: properly implement solomachine client state fields
impl<Chain, Counterparty> ClientStateFieldsGetter<Chain, Counterparty>
    for ProvideSolomachineClientState
where
    Chain: HasClientStateType<Counterparty, ClientState = SolomachineClientState>
        + HasHeightType<Height = Height>,
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
}
