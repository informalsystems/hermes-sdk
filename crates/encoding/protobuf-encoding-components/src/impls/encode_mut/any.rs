use cgp::prelude::HasErrorType;
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::decode_buffer::HasDecodeBufferType;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use prost_types::Any;

use crate::impls::encode_mut::proto_field::bytes::EncodeByteField;
use crate::impls::encode_mut::proto_field::string::EncodeStringField;

pub struct EncodeAny;

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Any> for EncodeAny
where
    Encoding: HasEncodeBufferType + HasErrorType,
    EncodeStringField<1>: MutEncoder<Encoding, Strategy, String>,
    EncodeByteField<2>: MutEncoder<Encoding, Strategy, Vec<u8>>,
{
    fn encode_mut(
        encoding: &Encoding,
        any: &Any,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        <EncodeStringField<1>>::encode_mut(encoding, &any.type_url, buffer)?;
        <EncodeByteField<2>>::encode_mut(encoding, &any.value, buffer)?;

        Ok(())
    }
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, Any> for EncodeAny
where
    Encoding: HasDecodeBufferType + HasErrorType,
    EncodeStringField<1>: MutDecoder<Encoding, Strategy, String>,
    EncodeByteField<2>: MutDecoder<Encoding, Strategy, Vec<u8>>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<Any, Encoding::Error> {
        let type_url = <EncodeStringField<1>>::decode_mut(encoding, buffer)?;
        let value = <EncodeByteField<2>>::decode_mut(encoding, buffer)?;

        Ok(Any { type_url, value })
    }
}
