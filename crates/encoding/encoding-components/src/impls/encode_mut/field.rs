use core::marker::PhantomData;

use cgp::prelude::HasErrorType;

use crate::traits::encode_mut::MutEncoder;
use crate::traits::field::{FieldGetter, GetField};
use crate::traits::types::encode_buffer::HasEncodeBufferType;

pub type EncodeField<Tag, InEncoder> = EncodeFieldWithGetter<GetField, Tag, InEncoder>;

pub struct EncodeFieldWithGetter<Getter, Tag, InEncoder>(pub PhantomData<(Getter, Tag, InEncoder)>);

impl<Encoding, Strategy, Value, Getter, Tag, InEncoder> MutEncoder<Encoding, Strategy, Value>
    for EncodeFieldWithGetter<Getter, Tag, InEncoder>
where
    Encoding: HasEncodeBufferType + HasErrorType,
    InEncoder: MutEncoder<Encoding, Strategy, Getter::Field>,
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
