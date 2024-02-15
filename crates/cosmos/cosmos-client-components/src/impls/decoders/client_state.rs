use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateDecoder, HasClientStateType,
};
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as ProtoClientState;
use ibc_proto::Protobuf;
use prost::{DecodeError, Message};
use tendermint_proto::Error as ProtoError;

use crate::impls::decoders::type_url::CanAssertTypeUrlMatches;
use crate::types::tendermint::TendermintClientState;

pub const TENDERMINT_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.tendermint.v1.ClientState";

pub struct DecodeTendermintClientStateProto;

impl<Chain, Counterparty> ClientStateDecoder<Chain, Counterparty>
    for DecodeTendermintClientStateProto
where
    Chain: HasClientStateType<Counterparty, ClientState = TendermintClientState>,
    Counterparty: CanRaiseError<ProtoError>,
{
    fn decode_client_state_bytes(
        client_state_bytes: &[u8],
    ) -> Result<TendermintClientState, Counterparty::Error> {
        let client_state = Protobuf::<ProtoClientState>::decode_vec(&client_state_bytes)
            .map_err(Counterparty::raise_error)?;

        Ok(client_state)
    }
}

pub struct DecodeTendermintClientStateFromAnyProto;

impl<Chain, Counterparty> ClientStateDecoder<Chain, Counterparty>
    for DecodeTendermintClientStateFromAnyProto
where
    Chain: HasClientStateType<Counterparty, ClientState = TendermintClientState>,
    Counterparty: CanRaiseError<DecodeError> + CanAssertTypeUrlMatches,
    DecodeTendermintClientStateProto: ClientStateDecoder<Chain, Counterparty>,
{
    fn decode_client_state_bytes(
        client_state_bytes: &[u8],
    ) -> Result<TendermintClientState, Counterparty::Error> {
        let any = Any::decode(client_state_bytes).map_err(Counterparty::raise_error)?;

        Counterparty::assert_type_url_matches(TENDERMINT_CLIENT_STATE_TYPE_URL, &any.type_url)?;

        let client_state = DecodeTendermintClientStateProto::decode_client_state_bytes(&any.value)?;

        Ok(client_state)
    }
}
