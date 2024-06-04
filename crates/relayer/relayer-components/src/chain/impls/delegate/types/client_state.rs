use core::marker::PhantomData;
use core::time::Duration;

use cgp_core::{Async, DelegateComponent};

use crate::chain::traits::types::client_state::{
    ClientStateFieldsGetter, HasClientStateType, ProvideClientStateType,
};
use crate::chain::traits::types::height::HasHeightType;

pub struct DelegateClientStateType<Components>(pub PhantomData<Components>);

impl<Chain, Counterparty, Components, Delegate> ProvideClientStateType<Chain, Counterparty>
    for DelegateClientStateType<Components>
where
    Chain: Async,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ProvideClientStateType<Chain, Counterparty>,
{
    type ClientState = Delegate::ClientState;
}

impl<Chain, Counterparty, Components, Delegate> ClientStateFieldsGetter<Chain, Counterparty>
    for DelegateClientStateType<Components>
where
    Chain: HasClientStateType<Counterparty> + HasHeightType,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ClientStateFieldsGetter<Chain, Counterparty>,
{
    fn client_state_latest_height(client_state: &Chain::ClientState) -> Chain::Height {
        Delegate::client_state_latest_height(client_state)
    }

    /// Whether or not the client is frozen
    fn client_state_is_frozen(client_state: &Chain::ClientState) -> bool {
        Delegate::client_state_is_frozen(client_state)
    }

    /// Check if the client state will expired when `elapsed` time has passed
    /// since the latest consensus state
    fn client_state_has_expired(client_state: &Chain::ClientState, elapsed: Duration) -> bool {
        Delegate::client_state_has_expired(client_state, elapsed)
    }
}
