use crate::chain::traits::queries::client_state::{CanQueryClientStateBytes, ClientStateQuerier};
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

        let client_state = Counterparty::decode_client_state_bytes(&client_state_bytes)?;

        Ok(client_state)
    }
}
