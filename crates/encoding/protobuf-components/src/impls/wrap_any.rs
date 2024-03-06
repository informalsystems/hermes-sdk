use cgp_core::HasErrorType;
use hermes_relayer_components::encode::traits::decoder::Decoder;
use hermes_relayer_components::encode::traits::encoded::HasEncodedType;
use hermes_relayer_components::encode::traits::encoder::Encoder;

use crate::impls::any::EncodeAsAnyProtobuf;
use crate::impls::from_context::EncodeFromContext;
use crate::types::any::Any;

pub struct EncodeWrapAny;

impl<Encoding, Value> Encoder<Encoding, Any<Value>> for EncodeWrapAny
where
    Encoding: HasEncodedType + HasErrorType,
    EncodeAsAnyProtobuf<EncodeFromContext>: Encoder<Encoding, Value>,
{
    fn encode(
        encoding: &Encoding,
        value: &Any<Value>,
    ) -> Result<Encoding::Encoded, Encoding::Error> {
        <EncodeAsAnyProtobuf<EncodeFromContext>>::encode(encoding, &value.value)
    }
}

impl<Encoding, Value> Decoder<Encoding, Any<Value>> for EncodeWrapAny
where
    Encoding: HasEncodedType + HasErrorType,
    EncodeAsAnyProtobuf<EncodeFromContext>: Decoder<Encoding, Value>,
{
    fn decode(
        encoding: &Encoding,
        encoded: &Encoding::Encoded,
    ) -> Result<Any<Value>, Encoding::Error> {
        let value = <EncodeAsAnyProtobuf<EncodeFromContext>>::decode(encoding, encoded)?;

        Ok(Any { value })
    }
}
