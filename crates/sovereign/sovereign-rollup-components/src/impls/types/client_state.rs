use core::time::Duration;

use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsGetter, HasClientStateType, ProvideClientStateType,
};
use hermes_relayer_components::chain::traits::types::height::HasHeightType;

use crate::types::client_state::WrappedSovereignClientState;
use crate::types::height::RollupHeight;

pub struct ProvideSovereignClientState;

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty>
    for ProvideSovereignClientState
where
    Chain: Async,
{
    type ClientState = WrappedSovereignClientState;
}

impl<Chain, Counterparty> ClientStateFieldsGetter<Chain, Counterparty>
    for ProvideSovereignClientState
where
    Chain: HasHeightType<Height = RollupHeight>
        + HasClientStateType<Counterparty, ClientState = WrappedSovereignClientState>,
{
    fn client_state_latest_height(client_state: &WrappedSovereignClientState) -> RollupHeight {
        RollupHeight {
            slot_number: client_state
                .sovereign_client_state
                .sovereign_params
                .latest_height
                .revision_height(),
        }
    }

    fn client_state_is_frozen(client_state: &WrappedSovereignClientState) -> bool {
        client_state.sovereign_client_state.is_frozen()
    }

    fn client_state_has_expired(
        client_state: &WrappedSovereignClientState,
        elapsed: Duration,
    ) -> bool {
        elapsed
            > client_state
                .sovereign_client_state
                .sovereign_params
                .trusting_period
    }
}
