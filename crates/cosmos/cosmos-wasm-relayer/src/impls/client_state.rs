use core::time::Duration;

use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsGetter, HasClientStateType, ProvideClientStateType,
};
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use ibc_relayer_types::core::ics02_client::client_state::ClientState;
use ibc_relayer_types::Height;

use crate::types::client_state::WrappedTendermintClientState;

pub struct ProvideWrappedTendermintClientState;

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty>
    for ProvideWrappedTendermintClientState
where
    Chain: Async,
{
    type ClientState = WrappedTendermintClientState;
}

impl<Chain, Counterparty> ClientStateFieldsGetter<Chain, Counterparty>
    for ProvideWrappedTendermintClientState
where
    Chain: HasClientStateType<Counterparty, ClientState = WrappedTendermintClientState>
        + HasHeightType<Height = Height>,
{
    fn client_state_latest_height(client_state: &WrappedTendermintClientState) -> Height {
        client_state.tendermint_client_state.latest_height
    }

    fn client_state_is_frozen(client_state: &WrappedTendermintClientState) -> bool {
        client_state.tendermint_client_state.is_frozen()
    }

    fn client_state_has_expired(
        client_state: &WrappedTendermintClientState,
        elapsed: Duration,
    ) -> bool {
        elapsed > client_state.tendermint_client_state.trusting_period
    }
}
