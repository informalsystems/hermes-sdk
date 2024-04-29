use core::time::Duration;

use cgp_core::prelude::*;

use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::chain::traits::types::height::HasHeightType;

#[derive_component(ClientStateTypeComponent, ProvideClientStateType<Chain>)]
pub trait HasClientStateType<Counterparty>: Async {
    /**
        The client state of the `Self` chain's client on the `Counterparty` chain
    */
    type ClientState: Async;
}

#[derive_component(RawClientStateTypeComponent, ProvideRawClientStateType<Chain>)]
pub trait HasRawClientStateType: Async {
    type RawClientState: Async;
}

#[derive_component(ClientStateFieldsGetterComponent, ClientStateFieldsGetter<Chain>)]
pub trait HasClientStateFields<Counterparty>:
    HasChainIdType + HasHeightType + HasClientStateType<Counterparty>
{
    /// The id of the chain referenced by this client
    fn client_state_chain_id(client_state: &Self::ClientState) -> &Self::ChainId;

    /// The latest height of the client
    fn client_state_latest_height(client_state: &Self::ClientState) -> &Self::Height;

    /// Whether or not the client is frozen
    fn client_state_is_frozen(client_state: &Self::ClientState) -> bool;

    /// Check if the client state will expired when `elapsed` time has passed
    /// since the latest consensus state
    fn client_state_has_expired(client_state: &Self::ClientState, elapsed: Duration) -> bool;
}
