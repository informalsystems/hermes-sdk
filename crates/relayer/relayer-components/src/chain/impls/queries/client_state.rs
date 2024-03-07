use core::marker::PhantomData;

use alloc::vec::Vec;
use cgp_core::{Async, CanRaiseError, DelegateComponent, HasErrorType};

use crate::chain::traits::queries::client_state::{
    AllClientStatesQuerier, CanQueryAllClientStatesBytes, CanQueryClientStateBytes,
    ClientStateQuerier,
};
use crate::chain::traits::types::client_state::HasClientStateType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::encode::traits::decoder::CanDecode;
use crate::encode::traits::encoded::HasEncodedType;
use crate::encode::traits::has_encoding::HasDefaultEncoding;
use crate::encode::types::via::Via;

pub struct DelegateQueryClientState<Components>(pub PhantomData<Components>);

impl<Chain, Counterparty, Components, Delegate> ClientStateQuerier<Chain, Counterparty>
    for DelegateQueryClientState<Components>
where
    Chain: HasIbcChainTypes<Counterparty> + HasErrorType,
    Counterparty: HasClientStateType<Chain>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ClientStateQuerier<Chain, Counterparty>,
{
    async fn query_client_state(
        chain: &Chain,
        client_id: &Chain::ClientId,
        height: &Chain::Height,
    ) -> Result<Counterparty::ClientState, Chain::Error> {
        Delegate::query_client_state(chain, client_id, height).await
    }
}

impl<Chain, Counterparty, Components, Delegate> AllClientStatesQuerier<Chain, Counterparty>
    for DelegateQueryClientState<Components>
where
    Chain: HasIbcChainTypes<Counterparty> + HasErrorType,
    Counterparty: HasClientStateType<Chain>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: AllClientStatesQuerier<Chain, Counterparty>,
{
    async fn query_all_client_states(
        chain: &Chain,
        height: &Chain::Height,
    ) -> Result<Vec<(Chain::ClientId, Counterparty::ClientState)>, Chain::Error> {
        Delegate::query_all_client_states(chain, height).await
    }
}

pub struct QueryAndDecodeClientStateVia<Wrapper>(pub PhantomData<Wrapper>);

impl<Chain, Counterparty, Encoding, Wrapper> ClientStateQuerier<Chain, Counterparty>
    for QueryAndDecodeClientStateVia<Wrapper>
where
    Chain: CanQueryClientStateBytes<Counterparty> + CanRaiseError<Encoding::Error>,
    Counterparty: HasClientStateType<Chain> + HasDefaultEncoding<Encoding = Encoding>,
    Encoding:
        HasEncodedType<Encoded = Vec<u8>> + CanDecode<Via<Wrapper, Counterparty::ClientState>>,
    Wrapper: Async,
{
    async fn query_client_state(
        chain: &Chain,
        client_id: &Chain::ClientId,
        height: &Chain::Height,
    ) -> Result<Counterparty::ClientState, Chain::Error> {
        let client_state_bytes = chain.query_client_state_bytes(client_id, height).await?;

        let client_state = Counterparty::default_encoding()
            .decode(&client_state_bytes)
            .map_err(Chain::raise_error)?;

        Ok(client_state.value)
    }
}

impl<Chain, Counterparty, Encoding, Wrapper> AllClientStatesQuerier<Chain, Counterparty>
    for QueryAndDecodeClientStateVia<Wrapper>
where
    Chain: CanQueryAllClientStatesBytes<Counterparty>,
    Counterparty: HasClientStateType<Chain> + HasDefaultEncoding<Encoding = Encoding>,
    Encoding:
        HasEncodedType<Encoded = Vec<u8>> + CanDecode<Via<Wrapper, Counterparty::ClientState>>,
    Wrapper: Async,
{
    async fn query_all_client_states(
        chain: &Chain,
        height: &Chain::Height,
    ) -> Result<Vec<(Chain::ClientId, Counterparty::ClientState)>, Chain::Error> {
        let encoding = Counterparty::default_encoding();
        let raw_entries = chain.query_all_client_states_bytes(height).await?;

        let entries = raw_entries
            .into_iter()
            .filter_map(|(client_id, client_state_bytes)| {
                let client_state = encoding.decode(&client_state_bytes).ok()?;

                Some((client_id, client_state.value))
            })
            .collect();

        Ok(entries)
    }
}
