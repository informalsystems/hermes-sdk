use core::time::Duration;

use cgp::core::Async;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsGetter, HasClientStateType, ProvideClientStateType,
};
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use ibc::core::client::types::Height;

use crate::types::client_state::WasmTendermintClientState;

pub struct ProvideWrappedTendermintClientState;

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty>
    for ProvideWrappedTendermintClientState
where
    Chain: Async,
{
    type ClientState = WasmTendermintClientState;
}

impl<Chain, Counterparty> ClientStateFieldsGetter<Chain, Counterparty>
    for ProvideWrappedTendermintClientState
where
    Chain: HasClientStateType<Counterparty, ClientState = WasmTendermintClientState>
        + HasHeightType<Height = Height>,
{
    fn client_state_latest_height(client_state: &WasmTendermintClientState) -> Height {
        client_state.tendermint_client_state.inner().latest_height
    }

    fn client_state_is_frozen(client_state: &WasmTendermintClientState) -> bool {
        client_state.tendermint_client_state.inner().is_frozen()
    }

    fn client_state_has_expired(
        client_state: &WasmTendermintClientState,
        elapsed: Duration,
    ) -> bool {
        elapsed > client_state.tendermint_client_state.inner().trusting_period
    }
}
