use core::marker::PhantomData;

use cgp_core::prelude::*;
use hermes_encoding_components::traits::decode::Decoder;
use hermes_encoding_components::traits::encode::Encoder;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;

use crate::impls::any::{DecodeAsAnyProtobuf, EncodeAsAnyProtobuf};
use crate::impls::from_context::EncodeFromContext;

pub struct EncodeViaAny<InStrategy>(pub PhantomData<InStrategy>);

impl<Encoding, Strategy, InStrategy, Value> Encoder<Encoding, Strategy, Value>
    for EncodeViaAny<InStrategy>
where
    Encoding: HasEncodedType + HasErrorType,
    EncodeAsAnyProtobuf<InStrategy, EncodeFromContext>: Encoder<Encoding, Strategy, Value>,
    InStrategy: Async,
{
    fn encode(encoding: &Encoding, value: &Value) -> Result<Encoding::Encoded, Encoding::Error> {
        <EncodeAsAnyProtobuf<InStrategy, EncodeFromContext>>::encode(encoding, value)
    }
}

impl<Encoding, Strategy, InStrategy, Value> Decoder<Encoding, Strategy, Value>
    for EncodeViaAny<InStrategy>
where
    Encoding: HasEncodedType + HasErrorType,
    DecodeAsAnyProtobuf<InStrategy, EncodeFromContext>: Decoder<Encoding, InStrategy, Value>,
    InStrategy: Async,
{
    fn decode(encoding: &Encoding, encoded: &Encoding::Encoded) -> Result<Value, Encoding::Error> {
        let value =
            <DecodeAsAnyProtobuf<InStrategy, EncodeFromContext>>::decode(encoding, encoded)?;

        Ok(value)
    }
}
