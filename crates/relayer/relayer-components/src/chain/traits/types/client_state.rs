use cgp_core::prelude::*;

use crate::chain::traits::types::height::HasHeightType;

#[derive_component(ClientStateTypeComponent, ProvideClientStateType<Chain>)]
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

#[derive_component(ClientStateDecoderComponent, ClientStateDecoder<Chain>)]
pub trait CanDecodeClientState<Counterparty>: HasClientStateType<Counterparty>
where
    Counterparty: HasErrorType,
{
    fn decode_client_state_bytes(
        client_state_bytes: &[u8],
    ) -> Result<Self::ClientState, Counterparty::Error>;
}
