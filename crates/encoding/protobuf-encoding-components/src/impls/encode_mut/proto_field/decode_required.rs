use core::marker::PhantomData;

use cgp::prelude::CanRaiseError;
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use prost::encoding::{check_wire_type, WireType};
use prost::DecodeError;

use crate::impls::encode_mut::chunk::{
    CanDecodeProtoChunks, HasProtoChunksDecodeBuffer, ProtoChunks,
};

pub struct DecodeRequiredProtoField<const TAG: u32, InEncoder>(pub PhantomData<InEncoder>);

#[derive(Debug)]
pub struct RequiredFieldTagNotFound {
    pub tag: u32,
}

impl<Encoding, Strategy, Value, InEncoder, const TAG: u32> MutDecoder<Encoding, Strategy, Value>
    for DecodeRequiredProtoField<TAG, InEncoder>
where
    Encoding: CanDecodeProtoChunks
        + HasProtoChunksDecodeBuffer
        + CanRaiseError<DecodeError>
        + CanRaiseError<RequiredFieldTagNotFound>,
    InEncoder: MutDecoder<Encoding, Strategy, Value>,
{
    fn decode_mut(
        encoding: &Encoding,
        chunks: &mut ProtoChunks<'_>,
    ) -> Result<Value, Encoding::Error> {
        let (wire_type, mut bytes) = chunks
            .get(&TAG)
            .ok_or_else(|| Encoding::raise_error(RequiredFieldTagNotFound { tag: TAG }))?;

        check_wire_type(WireType::LengthDelimited, *wire_type).map_err(Encoding::raise_error)?;

        let mut in_chunks = Encoding::decode_protochunks(&mut bytes)?;

        let value = InEncoder::decode_mut(encoding, &mut in_chunks)?;

        Ok(value)
    }
}
