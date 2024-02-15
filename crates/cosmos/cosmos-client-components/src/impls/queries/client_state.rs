use hermes_relayer_components::chain::traits::queries::client_state::{
    ClientStateQuerier, ClientStatesQuerier,
};
use hermes_relayer_components::chain::traits::types::client_state::CanDecodeClientState;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::Height;

use crate::traits::abci_query::CanQueryAbci;

pub struct QueryCosmosClientStateFromAbci;

pub const IBC_QUERY_PATH: &str = "store/ibc/key";

impl<Chain, Counterparty> ClientStateQuerier<Chain, Counterparty> for QueryCosmosClientStateFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = Height> + CanQueryAbci,
    Counterparty: CanDecodeClientState<Chain>,
{
    async fn query_client_state(
        chain: &Chain,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Counterparty::ClientState, Chain::Error> {
        let client_state_path = format!("clients/{client_id}/clientState");

        let client_state_bytes = chain
            .query_abci(IBC_QUERY_PATH, client_state_path.as_bytes(), height)
            .await?;

        let client_state = Counterparty::decode_client_state_bytes(&client_state_bytes)?;

        Ok(client_state)
    }
}

impl<Chain, Counterparty> ClientStatesQuerier<Chain, Counterparty>
    for QueryCosmosClientStateFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = Height> + CanQueryAbci,
    Counterparty: CanDecodeClientState<Chain>,
{
    async fn query_client_states(
        _chain: &Chain,
    ) -> Result<Vec<(ClientId, Counterparty::ClientState)>, Chain::Error> {
        todo!()
    }
}
