use cgp_core::{CanRaiseError, HasInner};

use crate::chain::traits::queries::client_state::{CanQueryClientState, ClientStateQuerier};
use crate::chain::traits::types::client_state::HasClientStateType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;

pub struct ForwardQueryClientState;

impl<Chain, InChain, Counterparty, ClientState> ClientStateQuerier<Chain, Counterparty>
    for ForwardQueryClientState
where
    Chain:
        HasInner<Inner = InChain> + CanRaiseError<InChain::Error> + HasIbcChainTypes<Counterparty>,
    InChain: CanQueryClientState<Counterparty, ClientId = Chain::ClientId, Height = Chain::Height>,
    Counterparty: HasClientStateType<Chain, ClientState = ClientState>
        + HasClientStateType<InChain, ClientState = ClientState>,
{
    async fn query_client_state(
        chain: &Chain,
        client_id: &Chain::ClientId,
        height: &Chain::Height,
    ) -> Result<ClientState, Chain::Error> {
        let client_state = chain
            .inner()
            .query_client_state(client_id, height)
            .await
            .map_err(Chain::raise_error)?;

        Ok(client_state)
    }
}
