use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use hermes_cosmos_client_components::encoding::components::CosmosEncodingComponents;
use hermes_protobuf_components::types::Any;
use hermes_protobuf_components::vendor::HasSchemaType;
use hermes_relayer_components::encode::traits::decoder::{CanDecode, Decoder};
use hermes_relayer_components::encode::traits::encoded::HasEncodedType;
use hermes_relayer_components::encode::traits::schema::HasSchema;
use hermes_relayer_components::encode::types::wrap::Wrap;
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as ProtoTendermintClientState;
use ibc_relayer_types::clients::ics07_tendermint::client_state::ClientState as TendermintClientState;

use crate::any_client::impls::decoders::client_state::UnknownClientStateType;
use crate::any_client::types::client_state::AnyClientState;

pub struct AnyClientEncoderComponents;

delegate_components! {
    AnyClientEncoderComponents {
        [
            Wrap<Any, TendermintClientState>,
            TendermintClientState,
            Any,
            ProtoTendermintClientState,
        ]:
            CosmosEncodingComponents,
        AnyClientState:
            AnyClientStateEncoder,
    }
}

pub struct AnyClientStateEncoder;

impl<Encoding> Decoder<Encoding, AnyClientState> for AnyClientStateEncoder
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
