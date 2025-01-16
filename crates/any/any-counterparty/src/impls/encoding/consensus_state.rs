use core::marker::PhantomData;

use cgp::core::error::CanRaiseAsyncError;
use hermes_cosmos_chain_components::types::tendermint::TendermintConsensusState;
use hermes_encoding_components::traits::convert::Converter;
use hermes_encoding_components::traits::decode::{CanDecode, Decoder};
use hermes_encoding_components::traits::schema::HasSchema;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use hermes_protobuf_encoding_components::vendor::HasSchemaType;

use crate::types::consensus_state::AnyConsensusState;

pub struct EncodeAnyConsensusState;

#[derive(Debug)]
pub struct UnknownConsensusStateType {
    pub type_url: String,
}

impl<Encoding, ConsensusState> Converter<Encoding, Any, ConsensusState> for EncodeAnyConsensusState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + HasSchemaType<Schema = &'static str>
        + CanDecode<ViaProtobuf, TendermintConsensusState>
        + HasSchema<TendermintConsensusState>
        + CanRaiseAsyncError<UnknownConsensusStateType>,
    ConsensusState: From<AnyConsensusState>,
{
    fn convert(encoding: &Encoding, any: &Any) -> Result<ConsensusState, Encoding::Error> {
        if &any.type_url == encoding.schema(PhantomData::<TendermintConsensusState>) {
            let consensus_state: TendermintConsensusState = encoding.decode(&any.value)?;

            Ok(AnyConsensusState::Tendermint(consensus_state).into())
        } else {
            Err(Encoding::raise_error(UnknownConsensusStateType {
                type_url: any.type_url.clone(),
            }))
        }
    }
}

impl<Encoding, Strategy, ConsensusState> Decoder<Encoding, Strategy, ConsensusState>
    for EncodeAnyConsensusState
where
    Self: Converter<Encoding, Any, ConsensusState>,
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + HasSchemaType<Schema = &'static str>
        + CanDecode<Strategy, TendermintConsensusState>
        + CanDecode<Strategy, Any>
        + HasSchema<TendermintConsensusState>
        + CanRaiseAsyncError<UnknownConsensusStateType>,
    ConsensusState: From<AnyConsensusState>,
{
    fn decode(encoding: &Encoding, encoded: &Vec<u8>) -> Result<ConsensusState, Encoding::Error> {
        let any: Any = encoding.decode(encoded)?;

        Self::convert(encoding, &any)
    }
}
