use cgp::prelude::*;

use crate::traits::decode_mut::{CanDecodeMut, MutDecoder, MutDecoderComponent};
use crate::traits::encode_mut::{CanEncodeMut, MutEncoder, MutEncoderComponent};
use crate::traits::types::encode_buffer::HasEncodeBufferType;

pub struct EncodeFields;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, Value> MutEncoder<Encoding, Strategy, Value> for EncodeFields
where
    Encoding: HasEncodeBufferType
        + HasAsyncErrorType
        + for<'a> CanEncodeMut<Strategy, Value::FieldsRef<'a>>,
    Value: ToFieldsRef,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&value.to_fields_ref(), buffer)?;

        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy, Value> MutDecoder<Encoding, Strategy, Value> for EncodeFields
where
    Encoding: CanDecodeMut<Strategy, Value::Fields>,
    Value: FromFields,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<Value, Encoding::Error> {
        let fields = encoding.decode_mut(buffer)?;

        Ok(Value::from_fields(fields))
    }
}
