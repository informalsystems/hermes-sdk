use core::marker::PhantomData;

use cgp_core::error::CanRaiseError;
use hermes_encoding_components::traits::convert::Converter;
use hermes_encoding_components::traits::decoder::{CanDecode, Decoder};
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::schema::HasSchema;
use hermes_protobuf_encoding_components::types::{Any, Protobuf};
use hermes_protobuf_encoding_components::vendor::HasSchemaType;
use ibc_relayer_types::clients::ics07_tendermint::client_state::ClientState as TendermintClientState;

use crate::types::client_state::AnyClientState;

pub struct EncodeAnyClientState;

#[derive(Debug)]
pub struct UnknownClientStateType {
    pub type_url: String,
}

impl<Encoding, ClientState> Converter<Encoding, Any, ClientState> for EncodeAnyClientState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + HasSchemaType<Schema = &'static str>
        + CanDecode<Protobuf, TendermintClientState>
        + HasSchema<TendermintClientState>
        + CanRaiseError<UnknownClientStateType>,
    ClientState: From<AnyClientState>,
{
    fn convert(encoding: &Encoding, any: &Any) -> Result<ClientState, Encoding::Error> {
        if &any.type_url == encoding.schema(PhantomData::<TendermintClientState>) {
            let client_state: TendermintClientState = encoding.decode(&any.value)?;

            Ok(AnyClientState::Tendermint(client_state).into())
        } else {
            Err(Encoding::raise_error(UnknownClientStateType {
                type_url: any.type_url.clone(),
            }))
        }
    }
}

impl<Encoding, Strategy, ClientState> Decoder<Encoding, Strategy, ClientState>
    for EncodeAnyClientState
where
    Self: Converter<Encoding, Any, ClientState>,
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + HasSchemaType<Schema = &'static str>
        + CanDecode<Strategy, TendermintClientState>
        + CanDecode<Strategy, Any>
        + HasSchema<TendermintClientState>
        + CanRaiseError<UnknownClientStateType>,
    ClientState: From<AnyClientState>,
{
    fn decode(encoding: &Encoding, encoded: &Vec<u8>) -> Result<ClientState, Encoding::Error> {
        let any: Any = encoding.decode(encoded)?;

        Self::convert(encoding, &any)
    }
}
