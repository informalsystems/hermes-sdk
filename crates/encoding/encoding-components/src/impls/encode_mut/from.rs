use core::marker::PhantomData;

use cgp::prelude::*;

use crate::traits::{HasDecodeBufferType, MutDecoder, MutDecoderComponent, Transformer};

pub struct DecodeFrom<Transform, InDecoder>(pub PhantomData<(Transform, InDecoder)>);

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy, Transform, Source, Target, InDecoder>
    MutDecoder<Encoding, Strategy, Target> for DecodeFrom<Transform, InDecoder>
where
    Encoding: HasDecodeBufferType + HasAsyncErrorType,
    InDecoder: MutDecoder<Encoding, Strategy, Source>,
    Transform: Transformer<From = Source, To = Target>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Target, Encoding::Error> {
        let source = InDecoder::decode_mut(encoding, buffer)?;
        Ok(Transform::transform(source))
    }
}
