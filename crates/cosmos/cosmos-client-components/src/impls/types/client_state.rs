use core::time::Duration;

use cgp_core::prelude::Async;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsGetter, HasClientStateType, ProvideClientStateType,
};
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::encode::types::via::Via;
use ibc_relayer_types::core::ics02_client::client_state::ClientState;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_relayer_types::Height;
use prost_types::Any;

use crate::types::tendermint::TendermintClientState;

pub struct ProvideTendermintClientState;

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty>
    for ProvideTendermintClientState
where
    Chain: Async,
{
    type ClientState = Via<Any, TendermintClientState>;
}

impl<Chain, Counterparty, ClientState> ClientStateFieldsGetter<Chain, Counterparty>
    for ProvideTendermintClientState
where
    Chain: HasChainIdType<ChainId = ChainId>
        + HasHeightType<Height = Height>
        + HasClientStateType<Counterparty, ClientState = ClientState>,
    ClientState: AsRef<TendermintClientState>,
{
    fn client_state_chain_id(client_state: &ClientState) -> &ChainId {
        &client_state.as_ref().chain_id
    }

    fn client_state_latest_height(client_state: &ClientState) -> &Height {
        &client_state.as_ref().latest_height
    }

    fn client_state_is_frozen(client_state: &ClientState) -> bool {
        client_state.as_ref().is_frozen()
    }

    fn client_state_has_expired(client_state: &ClientState, elapsed: Duration) -> bool {
        elapsed > client_state.as_ref().trusting_period
    }
}
