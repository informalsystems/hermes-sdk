use core::marker::PhantomData;

use cgp::prelude::HasAsyncErrorType;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;

pub struct EncodeOption<InEncoder>(pub PhantomData<InEncoder>);

impl<Encoding, Strategy, Value, InEncoder> MutEncoder<Encoding, Strategy, Option<Value>>
    for EncodeOption<InEncoder>
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
    InEncoder: MutEncoder<Encoding, Strategy, Value>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Option<Value>,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        if let Some(value) = value {
            InEncoder::encode_mut(encoding, value, buffer)?;
        }

        Ok(())
    }
}
