use core::time::Duration;

use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsGetter, HasClientStateType, ProvideClientStateType,
};
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_sovereign_rollup_components::types::height::RollupHeight;

use crate::sovereign::types::client_state::SovereignClientState;

pub struct ProvideSovereignClientState;

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty>
    for ProvideSovereignClientState
where
    Chain: Async,
{
    type ClientState = SovereignClientState;
}

impl<Chain, Counterparty> ClientStateFieldsGetter<Chain, Counterparty>
    for ProvideSovereignClientState
where
    Chain: HasHeightType<Height = RollupHeight>
        + HasClientStateType<Counterparty, ClientState = SovereignClientState>,
{
    fn client_state_latest_height(client_state: &SovereignClientState) -> RollupHeight {
        RollupHeight {
            slot_number: client_state.latest_height.revision_height(),
        }
    }

    fn client_state_is_frozen(client_state: &SovereignClientState) -> bool {
        client_state.is_frozen()
    }

    fn client_state_has_expired(client_state: &SovereignClientState, elapsed: Duration) -> bool {
        elapsed > client_state.da_params.trusting_period
    }
}
