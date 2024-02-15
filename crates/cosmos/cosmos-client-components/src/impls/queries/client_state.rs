use hermes_relayer_components::chain::traits::queries::client_state::{
    ClientStateQuerier, ClientStatesQuerier,
};
use hermes_relayer_components::chain::traits::types::client_state::CanDecodeClientState;
use hermes_relayer_components::chain::traits::types::client_state::CanDecodeClientStates;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

use ibc_proto::ibc::core::client::v1::QueryClientStatesRequest as ProtoQueryClientStatesRequest;
use ibc_relayer::chain::requests::{PageRequest, QueryClientStatesRequest};
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
    Counterparty: CanDecodeClientStates<Chain>,
{
    async fn query_client_states(
        chain: &Chain,
        height: &Height,
    ) -> Result<Vec<(ClientId, Counterparty::ClientState)>, Chain::Error> {
        let request = ProtoQueryClientStatesRequest::from(QueryClientStatesRequest {
            pagination: Some(PageRequest::all()),
        });

        let data = prost::Message::encode_to_vec(&request);

        let client_states_bytes = chain
            .query_abci("/ibc.core.client.v1.Query/ClientStates", &data, height)
            .await?;

        let client_states = Counterparty::decode_client_states_bytes(&client_states_bytes)?;

        Ok(client_states)
    }
}
