use core::iter::Iterator;
use core::str::FromStr;

use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::client_state::{
    AllClientStatesBytesQuerier, AllRawClientStatesQuerier, ClientStateBytesQuerier,
    RawClientStateQuerier,
};
use hermes_relayer_components::chain::traits::types::client_state::HasRawClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_proto::ibc::core::client::v1::{
    IdentifiedClientState, QueryClientStatesRequest as ProtoQueryClientStatesRequest,
    QueryClientStatesResponse,
};
use ibc_relayer::chain::requests::{PageRequest, QueryClientStatesRequest};
use ibc_relayer_types::core::ics24_host::error::ValidationError;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::Height;
use prost::{DecodeError, EncodeError, Message};
use prost_types::Any;

use crate::traits::abci_query::CanQueryAbci;

pub struct QueryCosmosClientStateFromAbci;

pub const IBC_QUERY_PATH: &str = "store/ibc/key";

impl<Chain, Counterparty> ClientStateBytesQuerier<Chain, Counterparty>
    for QueryCosmosClientStateFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = Height> + CanQueryAbci,
{
    async fn query_client_state_bytes(
        chain: &Chain,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Vec<u8>, Chain::Error> {
        let client_state_path = format!("clients/{client_id}/clientState");

        let client_state_bytes = chain
            .query_abci(IBC_QUERY_PATH, client_state_path.as_bytes(), height)
            .await?;

        Ok(client_state_bytes)
    }
}

impl<Chain, Counterparty> RawClientStateQuerier<Chain, Counterparty>
    for QueryCosmosClientStateFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = Height>
        + HasRawClientStateType<RawClientState = Any>
        + CanQueryAbci
        + CanRaiseError<DecodeError>,
{
    async fn query_raw_client_state(
        chain: &Chain,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Any, Chain::Error> {
        let client_state_path = format!("clients/{client_id}/clientState");

        let client_state_bytes = chain
            .query_abci(IBC_QUERY_PATH, client_state_path.as_bytes(), height)
            .await?;

        let client_state_any =
            Message::decode(client_state_bytes.as_ref()).map_err(Chain::raise_error)?;

        Ok(client_state_any)
    }
}

impl<Chain, Counterparty> AllRawClientStatesQuerier<Chain, Counterparty>
    for QueryCosmosClientStateFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = Height>
        + CanQueryAbci
        + HasRawClientStateType<RawClientState = Any>
        + CanRaiseError<DecodeError>
        + CanParseClientStateEntryToAny<Counterparty>,
{
    async fn query_all_raw_client_states(
        chain: &Chain,
        height: &Height,
    ) -> Result<Vec<(ClientId, Any)>, Chain::Error> {
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
            .filter_map(|entry| Chain::parse_client_state_entry_to_any(entry).ok())
            .collect();

        Ok(client_states)
    }
}

impl<Chain, Counterparty> AllClientStatesBytesQuerier<Chain, Counterparty>
    for QueryCosmosClientStateFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = Height>
        + CanQueryAbci
        + CanRaiseError<DecodeError>
        + CanParseClientStateEntryBytes<Counterparty>,
{
    async fn query_all_client_states_bytes(
        chain: &Chain,
        height: &Height,
    ) -> Result<Vec<(ClientId, Vec<u8>)>, Chain::Error> {
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
            .filter_map(|entry| Chain::parse_client_state_entry_bytes(entry).ok())
            .collect();

        Ok(client_states)
    }
}

pub trait CanParseClientStateEntryToAny<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
{
    fn parse_client_state_entry_to_any(
        entry: IdentifiedClientState,
    ) -> Result<(Self::ClientId, Any), Self::Error>;
}

impl<Chain, Counterparty> CanParseClientStateEntryToAny<Counterparty> for Chain
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId>
        + CanRaiseError<ValidationError>
        + CanRaiseError<&'static str>,
{
    fn parse_client_state_entry_to_any(
        entry: IdentifiedClientState,
    ) -> Result<(ClientId, Any), Chain::Error> {
        let client_id = ClientId::from_str(&entry.client_id).map_err(Chain::raise_error)?;

        let client_state_any = entry
            .client_state
            .ok_or_else(|| Chain::raise_error("expect client state field to be non-empty"))?;

        Ok((
            client_id,
            Any {
                type_url: client_state_any.type_url,
                value: client_state_any.value,
            },
        ))
    }
}

pub trait CanParseClientStateEntryBytes<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
{
    fn parse_client_state_entry_bytes(
        entry: IdentifiedClientState,
    ) -> Result<(Self::ClientId, Vec<u8>), Self::Error>;
}

impl<Chain, Counterparty> CanParseClientStateEntryBytes<Counterparty> for Chain
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId>
        + CanRaiseError<ValidationError>
        + CanRaiseError<EncodeError>
        + CanRaiseError<&'static str>,
{
    fn parse_client_state_entry_bytes(
        entry: IdentifiedClientState,
    ) -> Result<(ClientId, Vec<u8>), Chain::Error> {
        let client_id = ClientId::from_str(&entry.client_id).map_err(Chain::raise_error)?;

        let client_state_any = entry
            .client_state
            .ok_or_else(|| Chain::raise_error("expect client state field to be non-empty"))?;

        let client_state_bytes = Message::encode_to_vec(&client_state_any);

        Ok((client_id, client_state_bytes))
    }
}
