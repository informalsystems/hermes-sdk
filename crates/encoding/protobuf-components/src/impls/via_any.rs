use cgp_core::HasErrorType;
use hermes_relayer_components::encode::traits::decoder::Decoder;
use hermes_relayer_components::encode::traits::encoded::HasEncodedType;
use hermes_relayer_components::encode::traits::encoder::Encoder;
use hermes_relayer_components::encode::types::via::Via;
use prost_types::Any;

use crate::impls::any::EncodeAsAnyProtobuf;
use crate::impls::from_context::EncodeFromContext;

pub struct EncodeViaAny;

impl<Encoding, Value> Encoder<Encoding, Via<Any, Value>> for EncodeViaAny
where
    Encoding: HasEncodedType + HasErrorType,
    EncodeAsAnyProtobuf<EncodeFromContext>: Encoder<Encoding, Value>,
{
    fn encode(
        encoding: &Encoding,
        value: &Via<Any, Value>,
    ) -> Result<Encoding::Encoded, Encoding::Error> {
        <EncodeAsAnyProtobuf<EncodeFromContext>>::encode(encoding, &value.value)
    }
}

impl<Encoding, Value> Decoder<Encoding, Via<Any, Value>> for EncodeViaAny
where
    Encoding: HasEncodedType + HasErrorType,
    EncodeAsAnyProtobuf<EncodeFromContext>: Decoder<Encoding, Value>,
{
    fn decode(
        encoding: &Encoding,
        encoded: &Encoding::Encoded,
    ) -> Result<Via<Any, Value>, Encoding::Error> {
        let value = <EncodeAsAnyProtobuf<EncodeFromContext>>::decode(encoding, encoded)?;

        Ok(value.into())
    }
}
