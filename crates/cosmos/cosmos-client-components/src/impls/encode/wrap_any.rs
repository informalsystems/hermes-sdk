use cgp_core::HasErrorType;
use hermes_relayer_components::encode::traits::decoder::Decoder;
use hermes_relayer_components::encode::traits::encoded::HasEncodedType;
use hermes_relayer_components::encode::traits::encoder::Encoder;

use crate::impls::encode::any::EncodeAsAnyProtobuf;
use crate::impls::encode::from_context::EncodeFromContext;

#[derive(Default, Clone)]
pub struct WrapAny<Value> {
    pub value: Value,
}

pub struct EncodeWrapAny;

impl<Encoding, Value> Encoder<Encoding, WrapAny<Value>> for EncodeWrapAny
where
    Encoding: HasEncodedType + HasErrorType,
    EncodeAsAnyProtobuf<EncodeFromContext>: Encoder<Encoding, Value>,
{
    fn encode(
        encoding: &Encoding,
        value: &WrapAny<Value>,
    ) -> Result<Encoding::Encoded, Encoding::Error> {
        <EncodeAsAnyProtobuf<EncodeFromContext>>::encode(encoding, &value.value)
    }
}

impl<Encoding, Value> Decoder<Encoding, WrapAny<Value>> for EncodeWrapAny
where
    Encoding: HasEncodedType + HasErrorType,
    EncodeAsAnyProtobuf<EncodeFromContext>: Decoder<Encoding, Value>,
{
    fn decode(
        encoding: &Encoding,
        encoded: &Encoding::Encoded,
    ) -> Result<WrapAny<Value>, Encoding::Error> {
        let value = <EncodeAsAnyProtobuf<EncodeFromContext>>::decode(encoding, encoded)?;

        Ok(WrapAny { value })
    }
}
