use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::core::field::FieldGetter;
use cgp::prelude::*;

use crate::traits::encode_mut::{MutEncoder, MutEncoderComponent};
use crate::traits::types::encode_buffer::HasEncodeBufferType;

pub type EncodeField<Tag, InEncoder> = EncodeFieldWithGetter<UseContext, Tag, InEncoder>;

pub struct EncodeFieldWithGetter<Getter, Tag, InEncoder>(pub PhantomData<(Getter, Tag, InEncoder)>);

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, Value, Getter, Tag, InEncoder> MutEncoder<Encoding, Strategy, Value>
    for EncodeFieldWithGetter<Getter, Tag, InEncoder>
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
    InEncoder: MutEncoder<Encoding, Strategy, Getter::Value>,
    Getter: FieldGetter<Value, Tag>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let field = Getter::get_field(value, PhantomData);

        InEncoder::encode_mut(encoding, field, buffer)?;

        Ok(())
    }
}
