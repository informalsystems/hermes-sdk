use std::str::FromStr;

use cgp_core::prelude::Async;
use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::chain::HasChainTypes;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateDecoder, ClientStatesDecoder, HasClientStateType, ProvideClientStateType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::QueryClientStatesResponse;
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as ProtoClientState;
use ibc_proto::Protobuf;
use ibc_relayer_types::core::ics24_host::error::ValidationError as Ics24ValidationError;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use prost::{DecodeError, Message};
use tendermint_proto::Error as ProtoError;

use crate::types::tendermint::TendermintClientState;

pub struct ProvideTendermintClientState;

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty>
    for ProvideTendermintClientState
where
    Chain: Async,
{
    type ClientState = TendermintClientState;
}

pub const TENDERMINT_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.tendermint.v1.ClientState";

#[derive(Debug)]
pub struct TypeUrlMismatchError {
    pub expected_url: String,
    pub actual_url: String,
}

fn decode_client_state_any<Counterparty>(
    any: &Any,
) -> Result<TendermintClientState, Counterparty::Error>
where
    Counterparty: CanRaiseError<ProtoError>
        + CanRaiseError<DecodeError>
        + CanRaiseError<TypeUrlMismatchError>,
{
    if any.type_url != TENDERMINT_CLIENT_STATE_TYPE_URL {
        return Err(Counterparty::raise_error(TypeUrlMismatchError {
            expected_url: TENDERMINT_CLIENT_STATE_TYPE_URL.into(),
            actual_url: any.type_url.clone(),
        }));
    }

    let client_state =
        Protobuf::<ProtoClientState>::decode_vec(&any.value).map_err(Counterparty::raise_error)?;

    Ok(client_state)
}

impl<Chain, Counterparty> ClientStateDecoder<Chain, Counterparty> for ProvideTendermintClientState
where
    Chain: HasClientStateType<Counterparty, ClientState = TendermintClientState>,
    Counterparty: CanRaiseError<ProtoError>
        + CanRaiseError<DecodeError>
        + CanRaiseError<TypeUrlMismatchError>,
{
    fn decode_client_state_bytes(
        client_state_bytes: &[u8],
    ) -> Result<TendermintClientState, Counterparty::Error> {
        let any = Any::decode(client_state_bytes).map_err(Counterparty::raise_error)?;

        decode_client_state_any::<Counterparty>(&any)
    }
}

impl<Chain, Counterparty> ClientStatesDecoder<Chain, Counterparty> for ProvideTendermintClientState
where
    Chain: HasClientStateType<Counterparty, ClientState = TendermintClientState>,
    Counterparty: HasChainTypes
        + HasIbcChainTypes<Chain, ClientId = ClientId>
        + CanRaiseError<ProtoError>
        + CanRaiseError<DecodeError>
        + CanRaiseError<TypeUrlMismatchError>
        + CanRaiseError<Ics24ValidationError>,
{
    fn decode_client_states_bytes(
        client_state_bytes: &[u8],
    ) -> Result<Vec<(ClientId, TendermintClientState)>, Counterparty::Error> {
        let response = QueryClientStatesResponse::decode(client_state_bytes)
            .map_err(Counterparty::raise_error)?;

        response.client_states.iter().try_fold(
            Vec::with_capacity(response.client_states.len()),
            |mut acc, client_state| {
                let client_id = ClientId::from_str(&client_state.client_id)
                    .map_err(Counterparty::raise_error)?;

                if let Some(client_state_any) = &client_state.client_state {
                    if let Ok(client_state) =
                        decode_client_state_any::<Counterparty>(client_state_any)
                    {
                        acc.push((client_id, client_state))
                    } else {
                        // TODO(romac): Show warning?
                    }
                }

                Ok(acc)
            },
        )
    }
}
