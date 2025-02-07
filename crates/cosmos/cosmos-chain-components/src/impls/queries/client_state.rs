use core::iter::Iterator;
use core::str::FromStr;

use cgp::prelude::*;
use hermes_relayer_components::chain::traits::queries::client_state::{
    AllRawClientStatesQuerier, AllRawClientStatesQuerierComponent, RawClientStateQuerier,
    RawClientStateQuerierComponent, RawClientStateWithProofsQuerier,
    RawClientStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::types::client_state::HasRawClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc::core::client::types::Height;
use ibc::core::host::types::error::IdentifierError;
use ibc::core::host::types::identifiers::ClientId;
use ibc::cosmos_host::IBC_QUERY_PATH;
use ibc_proto::cosmos::base::query::v1beta1::PageRequest;
use ibc_proto::ibc::core::client::v1::{
    IdentifiedClientState, QueryClientStatesRequest as ProtoQueryClientStatesRequest,
    QueryClientStatesResponse,
};
use prost::{DecodeError, Message};
use prost_types::Any;

use crate::traits::abci_query::CanQueryAbci;

pub struct QueryCosmosClientStateFromAbci;

#[cgp_provider(RawClientStateQuerierComponent)]
impl<Chain, Counterparty> RawClientStateQuerier<Chain, Counterparty>
    for QueryCosmosClientStateFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = Height>
        + HasRawClientStateType<RawClientState = Any>
        + CanQueryAbci
        + CanRaiseAsyncError<DecodeError>,
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

#[cgp_provider(RawClientStateWithProofsQuerierComponent)]
impl<Chain, Counterparty> RawClientStateWithProofsQuerier<Chain, Counterparty>
    for QueryCosmosClientStateFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = Height>
        + HasRawClientStateType<RawClientState = Any>
        + CanQueryAbci
        + CanRaiseAsyncError<DecodeError>,
{
    async fn query_raw_client_state_with_proofs(
        chain: &Chain,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<(Any, Chain::CommitmentProof), Chain::Error> {
        let client_state_path = format!("clients/{client_id}/clientState");

        let (client_state_bytes, proofs) = chain
            .query_abci_with_proofs(IBC_QUERY_PATH, client_state_path.as_bytes(), height)
            .await?;

        let client_state_any =
            Message::decode(client_state_bytes.as_ref()).map_err(Chain::raise_error)?;

        Ok((client_state_any, proofs))
    }
}

#[cgp_provider(AllRawClientStatesQuerierComponent)]
impl<Chain, Counterparty> AllRawClientStatesQuerier<Chain, Counterparty>
    for QueryCosmosClientStateFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = Height>
        + CanQueryAbci
        + HasRawClientStateType<RawClientState = Any>
        + CanRaiseAsyncError<DecodeError>
        + CanParseClientStateEntryToAny<Counterparty>,
{
    async fn query_all_raw_client_states(
        chain: &Chain,
        height: &Height,
    ) -> Result<Vec<(ClientId, Any)>, Chain::Error> {
        let request = ProtoQueryClientStatesRequest {
            pagination: Some(PageRequest {
                limit: u32::MAX as u64,
                ..Default::default()
            }),
        };

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

pub trait CanParseClientStateEntryToAny<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasAsyncErrorType
{
    fn parse_client_state_entry_to_any(
        entry: IdentifiedClientState,
    ) -> Result<(Self::ClientId, Any), Self::Error>;
}

impl<Chain, Counterparty> CanParseClientStateEntryToAny<Counterparty> for Chain
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId>
        + CanRaiseAsyncError<IdentifierError>
        + CanRaiseAsyncError<&'static str>,
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
