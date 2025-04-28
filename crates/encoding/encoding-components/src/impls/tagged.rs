use hermes_prelude::*;

use crate::traits::{
    CanDecodeMut, CanEncodeMut, MutDecoder, MutDecoderComponent, MutEncoder, MutEncoderComponent,
};

pub struct EncodeTaggedField;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, Tag, Value> MutEncoder<Encoding, Strategy, Field<Tag, Value>>
    for EncodeTaggedField
where
    Encoding: CanEncodeMut<Strategy, Value>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Field<Tag, Value>,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&value.value, buffer)?;

        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy, Tag, Value> MutDecoder<Encoding, Strategy, Field<Tag, Value>>
    for EncodeTaggedField
where
    Encoding: CanDecodeMut<Strategy, Value>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<Field<Tag, Value>, Encoding::Error> {
        let value = encoding.decode_mut(buffer)?;

        Ok(value.into())
    }
}
