use core::marker::PhantomData;

use cgp::core::error::HasErrorType;

use crate::traits::encode_mut::MutEncoder;
use crate::traits::types::encode_buffer::HasEncodeBufferType;

pub struct CombineEncoders<Encoders>(pub PhantomData<Encoders>);

impl<Encoding, Strategy, Encoder, InEncoders, Value> MutEncoder<Encoding, Strategy, Value>
    for CombineEncoders<(Encoder, InEncoders)>
where
    Encoding: HasEncodeBufferType + HasErrorType,
    Encoder: MutEncoder<Encoding, Strategy, Value>,
    CombineEncoders<InEncoders>: MutEncoder<Encoding, Strategy, Value>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        Encoder::encode_mut(encoding, value, buffer)?;
        <CombineEncoders<InEncoders>>::encode_mut(encoding, value, buffer)?;

        Ok(())
    }
}

impl<Encoding, Strategy, Value> MutEncoder<Encoding, Strategy, Value> for CombineEncoders<()>
where
    Encoding: HasEncodeBufferType + HasErrorType,
{
    fn encode_mut(
        _encoding: &Encoding,
        _value: &Value,
        _buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        Ok(())
    }
}
