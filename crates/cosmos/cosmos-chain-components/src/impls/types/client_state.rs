use core::time::Duration;

use cgp::prelude::*;
use hermes_core::relayer_components::chain::traits::{
    ClientStateFieldsComponent, ClientStateFieldsGetter, ClientStateTypeComponent, HasChainIdType,
    HasClientStateType, HasHeightType, ProvideClientStateType, ProvideRawClientStateType,
    RawClientStateTypeComponent,
};
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::ChainId;
use prost_types::Any;

use crate::types::TendermintClientState;

pub struct ProvideTendermintClientState;

#[cgp_provider(ClientStateTypeComponent)]
impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty>
    for ProvideTendermintClientState
where
    Chain: Async,
{
    type ClientState = TendermintClientState;
}

pub struct ProvideAnyRawClientState;

#[cgp_provider(RawClientStateTypeComponent)]
impl<Chain> ProvideRawClientStateType<Chain> for ProvideAnyRawClientState
where
    Chain: Async,
{
    type RawClientState = Any;
}

#[cgp_provider(ClientStateFieldsComponent)]
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

    fn client_state_chain_id(client_state: &TendermintClientState) -> ChainId {
        client_state.chain_id.clone()
    }
}
