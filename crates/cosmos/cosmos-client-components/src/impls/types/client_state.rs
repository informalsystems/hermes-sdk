use cgp_core::prelude::Async;
use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateDecoder, HasClientStateType, ProvideClientStateType,
};
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as ProtoClientState;
use ibc_proto::Protobuf;
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

pub struct TypeUrlMismatchError {
    pub expected_url: String,
    pub actual_url: String,
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

        let client_state = Protobuf::<ProtoClientState>::decode_vec(&any.value)
            .map_err(Counterparty::raise_error)?;

        Ok(client_state)
    }
}
