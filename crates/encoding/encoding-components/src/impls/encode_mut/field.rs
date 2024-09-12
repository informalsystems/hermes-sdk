use core::marker::PhantomData;

use cgp::prelude::{HasErrorType, HasField};

use crate::traits::encode_mut::MutEncoder;
use crate::traits::types::encode_buffer::HasEncodeBufferType;

pub struct EncodeField<Tag, InEncoder>(pub PhantomData<(Tag, InEncoder)>);

impl<Encoding, Strategy, Value, Tag, InEncoder> MutEncoder<Encoding, Strategy, Value>
    for EncodeField<Tag, InEncoder>
where
    Encoding: HasEncodeBufferType + HasErrorType,
    Value: HasField<Tag>,
    InEncoder: MutEncoder<Encoding, Strategy, Value::Field>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let field = value.get_field(PhantomData);

        InEncoder::encode_mut(encoding, field, buffer)?;

        Ok(())
    }
}
