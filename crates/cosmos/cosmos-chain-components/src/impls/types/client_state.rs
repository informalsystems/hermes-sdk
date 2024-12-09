use core::time::Duration;

use cgp::prelude::Async;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsGetter, HasClientStateType, ProvideClientStateType, ProvideRawClientStateType,
};
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::ChainId;
use prost_types::Any;

use crate::types::tendermint::TendermintClientState;

pub struct ProvideTendermintClientState;

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty>
    for ProvideTendermintClientState
where
    Chain: Async,
{
    type ClientState = TendermintClientState;
}

pub struct ProvideAnyRawClientState;

impl<Chain> ProvideRawClientStateType<Chain> for ProvideAnyRawClientState
where
    Chain: Async,
{
    type RawClientState = Any;
}

impl<Chain, Counterparty> ClientStateFieldsGetter<Chain, Counterparty>
    for ProvideTendermintClientState
where
    Chain: HasChainIdType<ChainId = ChainId>
        + HasHeightType<Height = Height>
        + HasClientStateType<Counterparty, ClientState = TendermintClientState>,
{
    fn client_state_latest_height(client_state: &TendermintClientState) -> Height {
        client_state.latest_height
    }

    fn client_state_is_frozen(client_state: &TendermintClientState) -> bool {
        client_state.is_frozen()
    }

    fn client_state_has_expired(client_state: &TendermintClientState, elapsed: Duration) -> bool {
        elapsed > client_state.trusting_period
    }
}
