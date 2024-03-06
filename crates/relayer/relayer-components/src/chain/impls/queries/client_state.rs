use alloc::vec::Vec;

use crate::chain::traits::queries::client_state::{
    AllClientStatesQuerier, CanQueryAllClientStatesBytes, CanQueryClientStateBytes,
    ClientStateQuerier,
};
use crate::chain::traits::types::client_state::CanDecodeClientState;

pub struct QueryAndDecodeClientState;

impl<Chain, Counterparty> ClientStateQuerier<Chain, Counterparty> for QueryAndDecodeClientState
where
    Chain: CanQueryClientStateBytes<Counterparty>,
    Counterparty: CanDecodeClientState<Chain>,
{
    async fn query_client_state(
        chain: &Chain,
        client_id: &Chain::ClientId,
        height: &Chain::Height,
    ) -> Result<Counterparty::ClientState, Chain::Error> {
        let client_state_bytes = chain.query_client_state_bytes(client_id, height).await?;

        let client_state = Counterparty::decode_client_state_bytes(client_state_bytes)?;

        Ok(client_state)
    }
}

impl<Chain, Counterparty> AllClientStatesQuerier<Chain, Counterparty> for QueryAndDecodeClientState
where
    Chain: CanQueryAllClientStatesBytes<Counterparty>,
    Counterparty: CanDecodeClientState<Chain>,
{
    async fn query_all_client_states(
        chain: &Chain,
        height: &Chain::Height,
    ) -> Result<Vec<(Chain::ClientId, Counterparty::ClientState)>, Chain::Error> {
        let raw_entries = chain.query_all_client_states_bytes(height).await?;

        let entries = raw_entries
            .into_iter()
            .filter_map(|(client_id, client_state_bytes)| {
                let client_state =
                    Counterparty::decode_client_state_bytes(client_state_bytes).ok()?;

                Some((client_id, client_state))
            })
            .collect();

        Ok(entries)
    }
}
