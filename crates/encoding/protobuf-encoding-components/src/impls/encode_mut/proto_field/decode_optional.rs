use core::marker::PhantomData;

use hermes_encoding_components::traits::{MutDecoder, MutDecoderComponent};
use hermes_prelude::*;

use crate::impls::encode_mut::chunk::{
    CanDecodeProtoChunks, HasProtoChunksDecodeBuffer, InvalidWireType, ProtoChunks,
};

pub struct DecodeProtoOptionalField<const TAG: u32, InEncoder>(pub PhantomData<InEncoder>);

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy, Value, InEncoder, const TAG: u32> MutDecoder<Encoding, Strategy, Value>
    for DecodeProtoOptionalField<TAG, InEncoder>
where
    Encoding:
        CanDecodeProtoChunks + HasProtoChunksDecodeBuffer + CanRaiseAsyncError<InvalidWireType>,
    InEncoder: MutDecoder<Encoding, Strategy, Value>,
    Value: Default,
{
    fn decode_mut(
        encoding: &Encoding,
        chunks: &mut ProtoChunks<'_>,
    ) -> Result<Value, Encoding::Error> {
        match chunks.get(&TAG) {
            Some(chunk) => {
                let mut bytes = chunk.to_length_delimited().map_err(Encoding::raise_error)?;

                let mut in_chunks = Encoding::decode_protochunks(&mut bytes)?;

                let value = InEncoder::decode_mut(encoding, &mut in_chunks)?;

                Ok(value)
            }
            None => Ok(Default::default()),
        }
    }
}
