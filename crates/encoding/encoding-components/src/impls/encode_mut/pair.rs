use core::marker::PhantomData;

use cgp::core::component::UseContext;
use hermes_prelude::*;

use crate::traits::{
    HasDecodeBufferType, HasEncodeBufferType, MutDecoder, MutDecoderComponent, MutEncoder,
    MutEncoderComponent,
};

pub struct EncoderPair<EncoderA, EncoderB>(pub PhantomData<(EncoderA, EncoderB)>);

pub type EncodeCons<NextEncode> = EncoderPair<UseContext, NextEncode>;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, EncoderA, EncoderB, ValueA, ValueB>
    MutEncoder<Encoding, Strategy, (ValueA, ValueB)> for EncoderPair<EncoderA, EncoderB>
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
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

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy, EncoderA, EncoderB, ValueA, ValueB>
    MutDecoder<Encoding, Strategy, (ValueA, ValueB)> for EncoderPair<EncoderA, EncoderB>
where
    Encoding: HasDecodeBufferType + HasAsyncErrorType,
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
