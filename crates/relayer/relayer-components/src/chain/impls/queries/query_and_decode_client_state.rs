use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp_core::{Async, CanRaiseError};
use hermes_encoding_components::traits::decoder::CanDecode;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;

use crate::chain::traits::queries::client_state::{
    AllClientStatesQuerier, CanQueryAllClientStatesBytes, CanQueryClientStateBytes,
    ClientStateQuerier,
};
use crate::chain::traits::types::client_state::HasClientStateType;

pub struct QueryAndDecodeClientState<Strategy>(pub PhantomData<Strategy>);

impl<Chain, Counterparty, Encoding, Strategy> ClientStateQuerier<Chain, Counterparty>
    for QueryAndDecodeClientState<Strategy>
where
    Chain: CanQueryClientStateBytes<Counterparty> + CanRaiseError<Encoding::Error>,
    Counterparty: HasClientStateType<Chain> + HasDefaultEncoding<Encoding = Encoding>,
    Encoding: HasEncodedType<Encoded = Vec<u8>> + CanDecode<Strategy, Counterparty::ClientState>,
    Strategy: Async,
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

        Ok(client_state)
    }
}

impl<Chain, Counterparty, Encoding, Strategy> AllClientStatesQuerier<Chain, Counterparty>
    for QueryAndDecodeClientState<Strategy>
where
    Chain: CanQueryAllClientStatesBytes<Counterparty>,
    Counterparty: HasClientStateType<Chain> + HasDefaultEncoding<Encoding = Encoding>,
    Encoding: HasEncodedType<Encoded = Vec<u8>> + CanDecode<Strategy, Counterparty::ClientState>,
    Strategy: Async,
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

                Some((client_id, client_state))
            })
            .collect();

        Ok(entries)
    }
}
