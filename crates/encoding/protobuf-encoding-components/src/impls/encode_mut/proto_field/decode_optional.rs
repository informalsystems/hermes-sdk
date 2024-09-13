use core::marker::PhantomData;

use cgp::prelude::CanRaiseError;
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use prost::encoding::WireType;

use crate::impls::encode_mut::chunk::{
    CanDecodeProtoChunks, HasProtoChunksDecodeBuffer, InvalidWireType, ProtoChunk, ProtoChunks,
};

pub struct DecodeProtoOptionalField<const TAG: u32, InEncoder>(pub PhantomData<InEncoder>);

impl<Encoding, Strategy, Value, InEncoder, const TAG: u32> MutDecoder<Encoding, Strategy, Value>
    for DecodeProtoOptionalField<TAG, InEncoder>
where
    Encoding: CanDecodeProtoChunks + HasProtoChunksDecodeBuffer + CanRaiseError<InvalidWireType>,
    InEncoder: MutDecoder<Encoding, Strategy, Value>,
    Value: Default,
{
    fn decode_mut(
        encoding: &Encoding,
        chunks: &mut ProtoChunks<'_>,
    ) -> Result<Value, Encoding::Error> {
        match chunks.get(&TAG) {
            Some(ProtoChunk::LengthDelimited(mut bytes)) => {
                let mut in_chunks = Encoding::decode_protochunks(&mut bytes)?;

                let value = InEncoder::decode_mut(encoding, &mut in_chunks)?;

                Ok(value)
            }
            Some(chunk) => Err(Encoding::raise_error(InvalidWireType {
                expected: WireType::LengthDelimited,
                actual: chunk.wire_type(),
            })),
            None => Ok(Default::default()),
        }
    }
}
