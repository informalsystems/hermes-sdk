use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use hermes_cosmos_chain_components::encoding::components::CosmosEncodingComponents;
use hermes_encoding_components::impls::via_identity::{EncodeViaIdentity, Identity};
use hermes_encoding_components::traits::decoder::{CanDecode, Decoder};
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::schema::HasSchema;
use hermes_encoding_components::types::via::Via;
use hermes_protobuf_encoding_components::types::Any;
use hermes_protobuf_encoding_components::vendor::HasSchemaType;
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as ProtoTendermintClientState;
use ibc_relayer_types::clients::ics07_tendermint::client_state::ClientState as TendermintClientState;

use crate::any_client::types::client_state::AnyClientState;

#[derive(Debug)]
pub struct UnknownClientStateType {
    pub type_url: String,
}

pub struct AnyClientEncoderComponents;

delegate_components! {
    AnyClientEncoderComponents {
        [
            Via<Any, TendermintClientState>,
            TendermintClientState,
            Any,
            ProtoTendermintClientState,
        ]:
            CosmosEncodingComponents,
        AnyClientState: AnyClientStateEncoder,
        Via<Identity, AnyClientState>: EncodeViaIdentity,
    }
}

pub struct AnyClientStateEncoder;

impl<Encoding, ClientState> Decoder<Encoding, ClientState> for AnyClientStateEncoder
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + HasSchemaType<Schema = &'static str>
        + CanDecode<TendermintClientState>
        + CanDecode<Any>
        + HasSchema<TendermintClientState>
        + CanRaiseError<UnknownClientStateType>,
    ClientState: From<AnyClientState>,
{
    fn decode(encoding: &Encoding, encoded: &Vec<u8>) -> Result<ClientState, Encoding::Error> {
        let any: Any = encoding.decode(encoded)?;

        if &any.type_url == encoding.schema(PhantomData::<TendermintClientState>) {
            let client_state: TendermintClientState = encoding.decode(&any.value)?;
            Ok(AnyClientState::Tendermint(client_state).into())
        } else {
            Err(Encoding::raise_error(UnknownClientStateType {
                type_url: any.type_url,
            }))
        }
    }
}
