use core::iter::Iterator;
use core::str::FromStr;

use cgp_core::{CanRaiseError, HasErrorType};
use hermes_relayer_components::chain::traits::queries::client_state::{
    ClientStateQuerier, ClientStatesQuerier,
};
use hermes_relayer_components::chain::traits::types::client_state::{
    CanDecodeClientState, HasClientStateType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

use ibc_proto::ibc::core::client::v1::{
    IdentifiedClientState, QueryClientStatesRequest as ProtoQueryClientStatesRequest,
    QueryClientStatesResponse,
};
use ibc_relayer::chain::requests::{PageRequest, QueryClientStatesRequest};
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::Height;
use prost::{DecodeError, Message};

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
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = Height>
        + CanQueryAbci
        + CanRaiseError<DecodeError>
        + CanParseClientStateEntry<Counterparty>,
    Counterparty: CanDecodeClientState<Chain>,
{
    async fn query_client_states(
        chain: &Chain,
        height: &Height,
    ) -> Result<Vec<(ClientId, Counterparty::ClientState)>, Chain::Error> {
        let request = ProtoQueryClientStatesRequest::from(QueryClientStatesRequest {
            pagination: Some(PageRequest::all()),
        });

        let data = prost::Message::encode_to_vec(&request);

        let response = chain
            .query_abci("/ibc.core.client.v1.Query/ClientStates", &data, height)
            .await?;

        let response: QueryClientStatesResponse =
            QueryClientStatesResponse::decode(response.as_ref()).map_err(Chain::raise_error)?;

        let client_states = response
            .client_states
            .into_iter()
            .filter_map(|entry| Chain::parse_client_state_entry(entry).ok())
            .collect();

        Ok(client_states)
    }
}

pub trait CanParseClientStateEntry<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasClientStateType<Self>,
{
    fn parse_client_state_entry(
        entry: IdentifiedClientState,
    ) -> Result<(Self::ClientId, Counterparty::ClientState), Self::Error>;
}

impl<Chain, Counterparty> CanParseClientStateEntry<Counterparty> for Chain
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId> + CanRaiseError<&'static str>,
    Counterparty: CanDecodeClientState<Chain>,
{
    fn parse_client_state_entry(
        entry: IdentifiedClientState,
    ) -> Result<(ClientId, Counterparty::ClientState), Chain::Error> {
        // TODO: handle errors

        let client_id = ClientId::from_str(&entry.client_id).unwrap();

        let client_state_any = entry.client_state.unwrap();

        let client_state_bytes = {
            let mut buf = Vec::new();
            Message::encode(&client_state_any, &mut buf).unwrap();
            buf
        };

        let client_state = Counterparty::decode_client_state_bytes(&client_state_bytes)?;

        Ok((client_id, client_state))
    }
}
