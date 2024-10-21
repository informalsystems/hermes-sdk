use core::time::Duration;

use cgp::prelude::*;
pub use hermes_chain_type_components::traits::types::ibc::client_state::*;

use crate::traits::types::height::HasHeightType;

#[derive_component(RawClientStateTypeComponent, ProvideRawClientStateType<Chain>)]
pub trait HasRawClientStateType: Async {
    type RawClientState: Async;
}

#[derive_component(ClientStateFieldsGetterComponent, ClientStateFieldsGetter<Chain>)]
pub trait HasClientStateFields<Counterparty>:
    HasHeightType + HasClientStateType<Counterparty>
{
    /// The latest height of the client
    fn client_state_latest_height(client_state: &Self::ClientState) -> Self::Height;

    /// Whether or not the client is frozen
    fn client_state_is_frozen(client_state: &Self::ClientState) -> bool;

    /// Check if the client state will expired when `elapsed` time has passed
    /// since the latest consensus state
    fn client_state_has_expired(client_state: &Self::ClientState, elapsed: Duration) -> bool;
}
