use alloc::vec::Vec;
use core::marker::PhantomData;

use hermes_encoding_components::traits::{CanConvert, HasDefaultEncoding};
use hermes_encoding_components::types::AsBytes;
use hermes_prelude::*;

use crate::traits::{
    AllClientStatesQuerier, AllClientStatesQuerierComponent, CanQueryAllRawClientStates,
    CanQueryRawClientState, CanQueryRawClientStateWithProofs, ClientStateQuerier,
    ClientStateQuerierComponent, ClientStateWithProofsQuerier,
    ClientStateWithProofsQuerierComponent, HasClientStateType,
};

pub struct QueryAndConvertRawClientState;

#[cgp_provider(ClientStateQuerierComponent)]
impl<Chain, Counterparty, Encoding> ClientStateQuerier<Chain, Counterparty>
    for QueryAndConvertRawClientState
where
    Chain: CanQueryRawClientState<Counterparty> + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasClientStateType<Chain> + HasDefaultEncoding<AsBytes, Encoding = Encoding>,
    Encoding: Async + CanConvert<Chain::RawClientState, Counterparty::ClientState>,
{
    async fn query_client_state(
        chain: &Chain,
        _tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
        height: &Chain::Height,
    ) -> Result<Counterparty::ClientState, Chain::Error> {
        let raw_client_state = chain.query_raw_client_state(client_id, height).await?;

        let client_state = Counterparty::default_encoding()
            .convert(&raw_client_state)
            .map_err(Chain::raise_error)?;

        Ok(client_state)
    }
}

#[cgp_provider(ClientStateWithProofsQuerierComponent)]
impl<Chain, Counterparty, Encoding> ClientStateWithProofsQuerier<Chain, Counterparty>
    for QueryAndConvertRawClientState
where
    Chain: CanQueryRawClientStateWithProofs<Counterparty> + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasClientStateType<Chain> + HasDefaultEncoding<AsBytes, Encoding = Encoding>,
    Encoding: Async + CanConvert<Chain::RawClientState, Counterparty::ClientState>,
{
    async fn query_client_state_with_proofs(
        chain: &Chain,
        _tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
        height: &Chain::Height,
    ) -> Result<(Counterparty::ClientState, Chain::CommitmentProof), Chain::Error> {
        let (raw_client_state, proofs) = chain
            .query_raw_client_state_with_proofs(client_id, height)
            .await?;

        let client_state = Counterparty::default_encoding()
            .convert(&raw_client_state)
            .map_err(Chain::raise_error)?;

        Ok((client_state, proofs))
    }
}

#[cgp_provider(AllClientStatesQuerierComponent)]
impl<Chain, Counterparty, Encoding> AllClientStatesQuerier<Chain, Counterparty>
    for QueryAndConvertRawClientState
where
    Chain: CanQueryAllRawClientStates<Counterparty>,
    Counterparty: HasClientStateType<Chain> + HasDefaultEncoding<AsBytes, Encoding = Encoding>,
    Encoding: Async + CanConvert<Chain::RawClientState, Counterparty::ClientState>,
{
    async fn query_all_client_states(
        chain: &Chain,
        height: &Chain::Height,
    ) -> Result<Vec<(Chain::ClientId, Counterparty::ClientState)>, Chain::Error> {
        let encoding = Counterparty::default_encoding();
        let raw_entries = chain.query_all_raw_client_states(height).await?;

        let entries = raw_entries
            .into_iter()
            .filter_map(|(client_id, raw_client_state)| {
                let client_state = encoding.convert(&raw_client_state).ok()?;

                Some((client_id, client_state))
            })
            .collect();

        Ok(entries)
    }
}
