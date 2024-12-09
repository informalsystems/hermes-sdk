use core::time::Duration;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;
pub use hermes_chain_type_components::traits::types::ibc::client_state::*;

use crate::traits::types::height::HasHeightType;

#[cgp_component {
  name: RawClientStateTypeComponent,
  provider: ProvideRawClientStateType,
  context: Chain,
}]
pub trait HasRawClientStateType: Async {
    type RawClientState: Async;
}

#[cgp_component {
  name: ClientStateFieldsComponent,
  provider: ClientStateFieldsGetter,
  context: Chain,
}]
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

impl<Chain, Counterparty, Components, Delegate> ClientStateFieldsGetter<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasHeightType + HasClientStateType<Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ClientStateFieldsGetter<Chain, Counterparty>,
{
    fn client_state_latest_height(client_state: &Chain::ClientState) -> Chain::Height {
        Delegate::client_state_latest_height(client_state)
    }

    fn client_state_is_frozen(client_state: &Chain::ClientState) -> bool {
        Delegate::client_state_is_frozen(client_state)
    }

    fn client_state_has_expired(client_state: &Chain::ClientState, elapsed: Duration) -> bool {
        Delegate::client_state_has_expired(client_state, elapsed)
    }
}
