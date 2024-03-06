use core::marker::PhantomData;

use cgp_core::CanRaiseError;
use hermes_cosmos_client_components::impls::decoders::client_state::DecodeTendermintClientStateProto;
use hermes_cosmos_client_components::impls::decoders::client_state::TENDERMINT_CLIENT_STATE_TYPE_URL;
use hermes_cosmos_client_components::types::tendermint::TendermintClientState;
use hermes_relayer_components::chain::traits::types::client_state::ClientStateDecoder;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::encode::traits::decoder::CanDecode;
use hermes_relayer_components::encode::traits::decoder::Decoder;
use hermes_relayer_components::encode::traits::encoded::HasEncodedType;
use hermes_relayer_components::encode::traits::schema::HasSchema;
use hermes_relayer_components::encode::traits::schema::HasSchemaType;
use ibc_proto::google::protobuf::Any;
use prost::{DecodeError, Message};

use crate::any_client::contexts::tendermint::TendermintChain;
use crate::any_client::types::client_state::AnyClientState;

#[derive(Debug)]
pub struct UnknownClientStateType {
    pub type_url: String,
}

pub struct DecodeAnyClientState;

impl<Encoding> Decoder<Encoding, AnyClientState> for DecodeAnyClientState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + HasSchemaType<Schema = &'static str>
        + CanDecode<TendermintClientState>
        + CanDecode<Any>
        + HasSchema<TendermintClientState>
        + CanRaiseError<UnknownClientStateType>,
{
    fn decode(encoding: &Encoding, encoded: &Vec<u8>) -> Result<AnyClientState, Encoding::Error> {
        let any: Any = encoding.decode(encoded)?;

        if &any.type_url == encoding.schema(PhantomData::<TendermintClientState>) {
            let client_state: TendermintClientState = encoding.decode(&any.value)?;
            Ok(AnyClientState::Tendermint(client_state))
        } else {
            Err(Encoding::raise_error(UnknownClientStateType {
                type_url: any.type_url,
            }))
        }
    }
}

impl<Chain, Counterparty> ClientStateDecoder<Chain, Counterparty> for DecodeAnyClientState
where
    Chain: HasClientStateType<Counterparty, ClientState = AnyClientState>,
    Counterparty: CanRaiseError<DecodeError> + CanRaiseError<UnknownClientStateType>,
    DecodeTendermintClientStateProto: ClientStateDecoder<TendermintChain, Counterparty>,
{
    fn decode_client_state_bytes(
        client_state_bytes: Vec<u8>,
    ) -> Result<AnyClientState, Counterparty::Error> {
        let any = Any::decode(client_state_bytes.as_ref()).map_err(Counterparty::raise_error)?;

        match any.type_url.as_str() {
            TENDERMINT_CLIENT_STATE_TYPE_URL => {
                let client_state =
                    DecodeTendermintClientStateProto::decode_client_state_bytes(any.value)?;
                Ok(AnyClientState::Tendermint(client_state))
            }
            type_url => Err(Counterparty::raise_error(UnknownClientStateType {
                type_url: type_url.to_string(),
            })),
        }
    }
}
