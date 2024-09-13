use core::marker::PhantomData;

use cgp::prelude::HasErrorType;

use crate::traits::encode_mut::MutEncoder;
use crate::traits::field::FieldGetter;
use crate::traits::types::encode_buffer::HasEncodeBufferType;

pub struct EncodeField<Getter, InEncoder>(pub PhantomData<(Getter, InEncoder)>);

impl<Encoding, Strategy, Value, Getter, InEncoder> MutEncoder<Encoding, Strategy, Value>
    for EncodeField<Getter, InEncoder>
where
    Encoding: HasEncodeBufferType + HasErrorType,
    InEncoder: MutEncoder<Encoding, Strategy, Getter::Field>,
    Getter: FieldGetter<Value>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let field = Getter::get_field(value);

        InEncoder::encode_mut(encoding, field, buffer)?;

        Ok(())
    }
}
