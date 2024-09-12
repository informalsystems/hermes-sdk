use core::marker::PhantomData;

use hermes_encoding_components::traits::encode_mut::{CanEncodeMut, MutEncoder};
use prost::bytes::BufMut;
use prost::encoding::{encode_key, encode_varint, WireType};

use crate::traits::length::HasEncodedLength;

pub struct EncodeField<InStrategy, const TAG: u32>(pub PhantomData<InStrategy>);

impl<Encoding, Strategy, Value, InStrategy, const TAG: u32> MutEncoder<Encoding, Strategy, Value>
    for EncodeField<InStrategy, TAG>
where
    Encoding: CanEncodeMut<InStrategy, Value> + HasEncodedLength<InStrategy, Value>,
    Encoding::EncodeBuffer: BufMut,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encode_key(TAG, WireType::LengthDelimited, buffer);
        encode_varint(encoding.encoded_length(value), buffer);
        encoding.encode_mut(value, buffer)?;

        Ok(())
    }
}
