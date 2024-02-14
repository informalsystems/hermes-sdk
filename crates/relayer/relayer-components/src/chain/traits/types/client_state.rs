use cgp_core::Async;

use crate::chain::traits::types::height::HasHeightType;

pub trait HasClientStateType<Counterparty>: Async {
    /**
        The client state of the `Self` chain's client on the `Counterparty` chain
    */
    type ClientState: Async;
}

pub trait HasClientStateFields<Counterparty>:
    HasHeightType + HasClientStateType<Counterparty>
{
    fn client_state_latest_height(client_state: &Self::ClientState) -> &Self::Height;
}
