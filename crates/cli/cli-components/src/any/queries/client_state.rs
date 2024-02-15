use std::str::FromStr;

use cgp_core::CanRaiseError;
use cgp_core::HasErrorType;
use hermes_cosmos_client_components::impls::types::client_state::TypeUrlMismatchError;
use hermes_cosmos_client_components::traits::abci_query::CanQueryAbci;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::types::error::Error;
use hermes_relayer_components::chain::traits::queries::client_state::ClientStatesQuerier;
use hermes_relayer_components::chain::traits::types::chain::HasChainTypes;
use hermes_relayer_components::chain::traits::types::client_state::CanDecodeClientStates;
use hermes_relayer_components::chain::traits::types::client_state::ClientStateDecoder;
use hermes_relayer_components::chain::traits::types::client_state::ClientStatesDecoder;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::QueryClientStatesRequest as ProtoQueryClientStatesRequest;
use ibc_proto::ibc::core::client::v1::QueryClientStatesResponse;
use ibc_relayer::chain::requests::PageRequest;
use ibc_relayer::chain::requests::QueryClientStatesRequest;
use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics24_host::error::ValidationError as Ics24ValidationError;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use prost::{DecodeError, Message};
use tendermint_proto::Error as ProtoError;

use crate::any::client_state::AnyClientState;
use crate::any::component::AnyCounterpartyComponents;
use crate::any::counterparty::AnyCounterparty;

pub struct QueryAnyClientStatesFromChainHandle;

impl ClientStatesQuerier<CosmosChain, AnyCounterparty> for QueryAnyClientStatesFromChainHandle {
    async fn query_client_states(
        chain: &CosmosChain,
        height: &Height,
    ) -> Result<Vec<(ClientId, AnyClientState)>, Error> {
        let request = ProtoQueryClientStatesRequest::from(QueryClientStatesRequest {
            pagination: Some(PageRequest::all()),
        });

        let data = Message::encode_to_vec(&request);

        let client_states_bytes = chain
            .query_abci("/ibc.core.client.v1.Query/ClientStates", &data, height)
            .await?;

        let client_states =
            <AnyCounterparty as CanDecodeClientStates<CosmosChain>>::decode_client_states_bytes(
                &client_states_bytes,
            )
            .unwrap();

        Ok(client_states)
    }
}

impl<Chain, Counterparty> ClientStateDecoder<Chain, Counterparty> for AnyCounterpartyComponents
where
    Chain: HasClientStateType<Counterparty, ClientState = AnyClientState>,
    Counterparty: HasErrorType
        + CanRaiseError<ProtoError>
        + CanRaiseError<DecodeError>
        + CanRaiseError<TypeUrlMismatchError>
        + CanRaiseError<UnknownClientStateType>,
{
    fn decode_client_state_bytes(
        client_state_bytes: &[u8],
    ) -> Result<AnyClientState, Counterparty::Error> {
        let any = Any::decode(client_state_bytes).map_err(Counterparty::raise_error)?;
        let client_state =
            decode_client_state_any::<Counterparty>(&any)?.map_err(Counterparty::raise_error)?;
        Ok(client_state)
    }
}

impl<Chain, Counterparty> ClientStatesDecoder<Chain, Counterparty> for AnyCounterpartyComponents
where
    Chain: HasClientStateType<Counterparty, ClientState = AnyClientState>,
    Counterparty: HasErrorType
        + HasChainTypes
        + HasIbcChainTypes<Chain, ClientId = ClientId>
        + CanRaiseError<ProtoError>
        + CanRaiseError<DecodeError>
        + CanRaiseError<Ics24ValidationError>
        + CanRaiseError<TypeUrlMismatchError>,
{
    fn decode_client_states_bytes(
        client_state_bytes: &[u8],
    ) -> Result<Vec<(ClientId, AnyClientState)>, Counterparty::Error> {
        let response = QueryClientStatesResponse::decode(client_state_bytes)
            .map_err(Counterparty::raise_error)?;

        response.client_states.iter().try_fold(
            Vec::with_capacity(response.client_states.len()),
            |mut acc, client_state| {
                let client_id = ClientId::from_str(&client_state.client_id)
                    .map_err(Counterparty::raise_error)?;

                if let Some(client_state_any) = &client_state.client_state {
                    match decode_client_state_any::<Counterparty>(client_state_any)? {
                        Ok(client_state) => acc.push((client_id, client_state)),
                        Err(e) => tracing::warn!(
                            "found client with unknown client state type: {}",
                            e.type_url
                        ),
                    }
                }

                Ok(acc)
            },
        )
    }
}

#[derive(Debug)]
pub struct UnknownClientStateType {
    pub type_url: String,
}

fn decode_client_state_any<Counterparty>(
    any: &Any,
) -> Result<Result<AnyClientState, UnknownClientStateType>, Counterparty::Error>
where
    Counterparty: HasErrorType
        + CanRaiseError<ProtoError>
        + CanRaiseError<DecodeError>
        + CanRaiseError<TypeUrlMismatchError>,
{
    use hermes_cosmos_client_components::impls::types::client_state::decode_tendermint_client_state;
    use hermes_cosmos_client_components::impls::types::client_state::TENDERMINT_CLIENT_STATE_TYPE_URL;

    match any.type_url.as_str() {
        TENDERMINT_CLIENT_STATE_TYPE_URL => {
            let client_state = decode_tendermint_client_state::<Counterparty>(any)?;
            Ok(Ok(AnyClientState::Tendermint(client_state)))
        }
        type_url => Ok(Err(UnknownClientStateType {
            type_url: type_url.to_string(),
        })),
    }
}
