use alloc::vec::Vec;

use cgp_core::error::CanRaiseError;
use cgp_core::Async;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;

use crate::chain::traits::queries::client_state::{
    AllClientStatesQuerier, CanQueryAllRawClientStates, CanQueryRawClientState,
    CanQueryRawClientStateWithProofs, ClientStateQuerier, ClientStateWithProofsQuerier,
};
use crate::chain::traits::types::client_state::HasClientStateType;

pub struct QueryAndConvertRawClientState;

impl<Chain, Counterparty, Encoding> ClientStateQuerier<Chain, Counterparty>
    for QueryAndConvertRawClientState
where
    Chain: CanQueryRawClientState<Counterparty> + CanRaiseError<Encoding::Error>,
    Counterparty: HasClientStateType<Chain> + HasDefaultEncoding<Encoding = Encoding>,
    Encoding: Async + CanConvert<Chain::RawClientState, Counterparty::ClientState>,
{
    async fn query_client_state(
        chain: &Chain,
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

impl<Chain, Counterparty, Encoding> ClientStateWithProofsQuerier<Chain, Counterparty>
    for QueryAndConvertRawClientState
where
    Chain: CanQueryRawClientStateWithProofs<Counterparty> + CanRaiseError<Encoding::Error>,
    Counterparty: HasClientStateType<Chain> + HasDefaultEncoding<Encoding = Encoding>,
    Encoding: Async + CanConvert<Chain::RawClientState, Counterparty::ClientState>,
{
    async fn query_client_state_with_proofs(
        chain: &Chain,
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

impl<Chain, Counterparty, Encoding> AllClientStatesQuerier<Chain, Counterparty>
    for QueryAndConvertRawClientState
where
    Chain: CanQueryAllRawClientStates<Counterparty>,
    Counterparty: HasClientStateType<Chain> + HasDefaultEncoding<Encoding = Encoding>,
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
