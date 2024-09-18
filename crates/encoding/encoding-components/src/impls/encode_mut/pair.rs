use core::marker::PhantomData;

use cgp::core::error::HasErrorType;

use crate::impls::with_context::WithContext;
use crate::traits::decode_mut::MutDecoder;
use crate::traits::encode_mut::MutEncoder;
use crate::traits::types::decode_buffer::HasDecodeBufferType;
use crate::traits::types::encode_buffer::HasEncodeBufferType;

pub struct EncoderPair<EncoderA, EncoderB>(pub PhantomData<(EncoderA, EncoderB)>);

pub type EncodeCons<NextEncode> = EncoderPair<WithContext, NextEncode>;

impl<Encoding, Strategy, EncoderA, EncoderB, ValueA, ValueB>
    MutEncoder<Encoding, Strategy, (ValueA, ValueB)> for EncoderPair<EncoderA, EncoderB>
where
    Encoding: HasEncodeBufferType + HasErrorType,
    EncoderA: MutEncoder<Encoding, Strategy, ValueA>,
    EncoderB: MutEncoder<Encoding, Strategy, ValueB>,
{
    fn encode_mut(
        encoding: &Encoding,
        (a, b): &(ValueA, ValueB),
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        EncoderA::encode_mut(encoding, a, buffer)?;
        EncoderB::encode_mut(encoding, b, buffer)?;

        Ok(())
    }
}

impl<Encoding, Strategy, EncoderA, EncoderB, ValueA, ValueB>
    MutDecoder<Encoding, Strategy, (ValueA, ValueB)> for EncoderPair<EncoderA, EncoderB>
where
    Encoding: HasDecodeBufferType + HasErrorType,
    EncoderA: MutDecoder<Encoding, Strategy, ValueA>,
    EncoderB: MutDecoder<Encoding, Strategy, ValueB>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<(ValueA, ValueB), Encoding::Error> {
        let a = EncoderA::decode_mut(encoding, buffer)?;
        let b = EncoderB::decode_mut(encoding, buffer)?;

        Ok((a, b))
    }
}
