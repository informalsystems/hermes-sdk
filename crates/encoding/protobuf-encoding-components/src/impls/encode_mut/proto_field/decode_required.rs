use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_encoding_components::traits::{MutDecoder, MutDecoderComponent};

use crate::impls::encode_mut::chunk::{
    CanDecodeProtoChunks, HasProtoChunksDecodeBuffer, InvalidWireType, ProtoChunks,
};

pub struct DecodeRequiredProtoField<const TAG: u32, InEncoder>(pub PhantomData<InEncoder>);

#[derive(Debug)]
pub struct RequiredFieldTagNotFound {
    pub tag: u32,
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy, Value, InEncoder, const TAG: u32> MutDecoder<Encoding, Strategy, Value>
    for DecodeRequiredProtoField<TAG, InEncoder>
where
    Encoding: CanDecodeProtoChunks
        + HasProtoChunksDecodeBuffer
        + CanRaiseAsyncError<RequiredFieldTagNotFound>
        + CanRaiseAsyncError<InvalidWireType>,
    InEncoder: MutDecoder<Encoding, Strategy, Value>,
{
    fn decode_mut(
        encoding: &Encoding,
        chunks: &mut ProtoChunks<'_>,
    ) -> Result<Value, Encoding::Error> {
        let chunk = chunks
            .get(&TAG)
            .ok_or_else(|| Encoding::raise_error(RequiredFieldTagNotFound { tag: TAG }))?;

        let mut bytes = chunk.to_length_delimited().map_err(Encoding::raise_error)?;

        let mut in_chunks = Encoding::decode_protochunks(&mut bytes)?;

        let value = InEncoder::decode_mut(encoding, &mut in_chunks)?;

        Ok(value)
    }
}
