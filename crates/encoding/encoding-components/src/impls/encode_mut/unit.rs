use cgp::core::error::HasAsyncErrorType;

use crate::traits::decode_mut::MutDecoder;
use crate::traits::encode_mut::MutEncoder;
use crate::traits::types::decode_buffer::HasDecodeBufferType;
use crate::traits::types::encode_buffer::HasEncodeBufferType;

pub struct EncodeNothing;

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, ()> for EncodeNothing
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
{
    fn encode_mut(
        _encoding: &Encoding,
        _value: &(),
        _buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        Ok(())
    }
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, ()> for EncodeNothing
where
    Encoding: HasDecodeBufferType + HasAsyncErrorType,
{
    fn decode_mut(
        _encoding: &Encoding,
        _buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<(), Encoding::Error> {
        Ok(())
    }
}
