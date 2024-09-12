use core::marker::PhantomData;

use crate::traits::decode_mut::{CanDecodeMut, MutDecoder};
use crate::traits::transform::Transformer;

pub struct DecodeFrom<Transform>(pub PhantomData<Transform>);

impl<Encoding, Strategy, Transform, Source, Target> MutDecoder<Encoding, Strategy, Target>
    for DecodeFrom<Transform>
where
    Encoding: CanDecodeMut<Strategy, Source>,
    Transform: Transformer<From = Source, To = Target>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Target, Encoding::Error> {
        let source = encoding.decode_mut(buffer)?;
        Ok(Transform::transform(source))
    }
}
