use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cosmos_chain_components::types::tendermint::TendermintClientState;
use hermes_encoding_components::traits::convert::{Converter, ConverterComponent};
use hermes_encoding_components::traits::decode::{CanDecode, Decoder};
use hermes_encoding_components::traits::schema::HasSchema;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_protobuf_encoding_components::components::DecoderComponent;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use hermes_protobuf_encoding_components::vendor::HasSchemaType;

use crate::types::client_state::AnyClientState;

pub struct EncodeAnyClientState;

#[derive(Debug)]
pub struct UnknownClientStateType {
    pub type_url: String,
}

#[cgp_provider(ConverterComponent)]
impl<Encoding, ClientState> Converter<Encoding, Any, ClientState> for EncodeAnyClientState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + HasSchemaType<Schema = &'static str>
        + CanDecode<ViaProtobuf, TendermintClientState>
        + HasSchema<TendermintClientState>
        + CanRaiseAsyncError<UnknownClientStateType>,
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

#[cgp_provider(DecoderComponent)]
impl<Encoding, Strategy, ClientState> Decoder<Encoding, Strategy, ClientState>
    for EncodeAnyClientState
where
    Self: Converter<Encoding, Any, ClientState>,
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + HasSchemaType<Schema = &'static str>
        + CanDecode<Strategy, TendermintClientState>
        + CanDecode<Strategy, Any>
        + HasSchema<TendermintClientState>
        + CanRaiseAsyncError<UnknownClientStateType>,
    ClientState: From<AnyClientState>,
{
    fn decode(encoding: &Encoding, encoded: &Vec<u8>) -> Result<ClientState, Encoding::Error> {
        let any: Any = encoding.decode(encoded)?;

        Self::convert(encoding, &any)
    }
}
